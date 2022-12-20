use std::collections::HashMap;
use std::{env, str, fmt};
use rdkafka::{
  producer::{BaseRecord, ThreadedProducer},
  consumer::{BaseConsumer, Consumer},
  ClientConfig, Message,
  admin::{AdminClient, TopicReplication, NewTopic, AdminOptions},
  client::DefaultClientContext,
};
use message_box::MessageBox;
use std::time::Duration;
use log::info;

use serde::{Deserialize};
use crate::core::ChainConfig;
use crate::core::KafkaConfig;

#[derive(Clone, Debug, Deserialize)]
pub struct SignatureProcess {
  chain_config: ChainConfig,
  kafka_config: KafkaConfig,
  identity: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Coin {
  SRI,
  BTC,
  ETH,
  XMR,
}

impl fmt::Display for Coin {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Coin::SRI => write!(f, "SRI"),
      Coin::BTC => write!(f, "BTC"),
      Coin::ETH => write!(f, "ETH"),
      Coin::XMR => write!(f, "XMR"),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum MessageType {
  CoordinatorPubkeyToProcessor,
  CoordinatorGeneralMessageToProcessor,
  CoordinatorSecureMessageToProcessor,
  ProcessorPubkeyToCoordinator,
  ProcessorGeneralMessageToCoordinator,
  ProcessorSecureMessageToCoordinator,
  Default,
}

impl fmt::Display for MessageType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      MessageType::CoordinatorPubkeyToProcessor => write!(f, "coordinator_pubkey_to_processor"),
      MessageType::CoordinatorGeneralMessageToProcessor => write!(f, "coordinator_general_message_to_processor"),
      MessageType::CoordinatorSecureMessageToProcessor => write!(f, "coordinator_secure_message_to_processor"),
      MessageType::ProcessorPubkeyToCoordinator => write!(f, "processor_pubkey_to_coordinator"),
      MessageType::ProcessorGeneralMessageToCoordinator => write!(f, "processor_general_message_to_coordinator"),
      MessageType::ProcessorSecureMessageToCoordinator => write!(f, "processor_secure_message_to_coordinator"),
      MessageType::Default => write!(f, "Default"),
    }
  }
}

fn parse_message_type(message_type: &str) -> MessageType {
  let mut msg_type = MessageType::Default;
  match message_type {
    "coordinator_pubkey_to_processor" => {
      msg_type = MessageType::CoordinatorPubkeyToProcessor;
    }
    "coordinator_general_message_to_processor" => {
      msg_type = MessageType::CoordinatorGeneralMessageToProcessor;
    }
    "coordinator_secure_message_to_processor" => {
      msg_type = MessageType::CoordinatorSecureMessageToProcessor;
    }
    "processor_pubkey_to_coordinator" => {
      msg_type = MessageType::ProcessorPubkeyToCoordinator;
    }
    "processor_general_message_to_coordinator" => {
      msg_type = MessageType::ProcessorGeneralMessageToCoordinator;
    }
    "processor_secure_message_to_coordinator" => {
      msg_type = MessageType::ProcessorSecureMessageToCoordinator;
    }
    _ => {}
  }
  msg_type
}

// Configuration for admin client to check / initialize topics
fn create_config(kafka_config: &KafkaConfig) -> ClientConfig {
  let mut config = ClientConfig::new();
  config.set("bootstrap.servers", format!("{}:{}", kafka_config.host, kafka_config.port));
  config
}

// Creates admin client used to check / initialize topics
fn create_admin_client(kafka_config: &KafkaConfig) -> AdminClient<DefaultClientContext> {
  create_config(kafka_config).create().expect("admin client creation failed")
}

// SignatureProcess communicates General & Secure Messages using Kafka
// General Messages will contain communicated pubkeys & general messages
// General Messages are contained in partition 0
// Secure Messages are contained in parition 1
impl SignatureProcess {
  pub fn new(chain_config: ChainConfig, kafka_config: KafkaConfig, identity: String) -> Self {
    info!("New Signature Process");
    let chain_config = chain_config;
    let kafka_config = kafka_config;
    Self { chain_config: chain_config, identity: identity, kafka_config: kafka_config }
  }

  pub async fn run(self) {
    info!("Starting Signature Process");

    // Check/initialize kakfa topics
    let j = serde_json::to_string(&self.chain_config).unwrap();
    let topic_ref: HashMap<String, bool> = serde_json::from_str(&j).unwrap();

    let admin_client = create_admin_client(&self.kafka_config);
    let opts = AdminOptions::new().operation_timeout(Some(Duration::from_secs(1)));

    // Loop through each coin & initialize each kakfa topic
    for (_key, _value) in topic_ref.into_iter() {
      let mut topic: String = "".to_string();
      topic.push_str(&self.identity);
      let topic_ref = &mut String::from(&_key).to_lowercase();
      topic.push_str("_");
      topic.push_str(topic_ref);

      let initialized_topic = NewTopic {
        name: &topic,
        num_partitions: 2,
        replication: TopicReplication::Fixed(1),
        config: Vec::new(),
      };

      admin_client.create_topics(&[initialized_topic], &opts).await.expect("topic creation failed");
    }

    // Create Hashmap based on coins
    let coin_hashmap = create_coin_hashmap(&self.chain_config);

    // Initialize consumers to read the processor pubkey & general test messages on partition 0
    consume_general_messages_from_processor(&self.kafka_config, &self.identity, &coin_hashmap);

    // Initialize producer to send coordinator pubkey to processors on general partition
    produce_coordinator_pubkey(&self.kafka_config, &self.identity, &coin_hashmap);

    // Wait to receive all Processer Pubkeys
    process_received_pubkeys(&coin_hashmap).await;

    // Initialize consumer used to read secure test messages from processors on secure partition
    consume_processor_secure_test_message(&self.kafka_config, &self.identity, &coin_hashmap);

    // Initialize a producer that sends a general & secure test message
    produce_general_and_secure_test_message(&self.kafka_config, &self.identity, &coin_hashmap)
      .await;
  }

  fn stop(self) {
    info!("Stopping Signature Process");
  }
}

// Initialize consumers to read the processor pubkey & general test messages on partition 0
fn consume_general_messages_from_processor(
  kafka_config: &KafkaConfig,
  identity: &str,
  coin_hashmap: &HashMap<Coin, bool>,
) {
  let hashmap_clone = coin_hashmap.clone();

  // Loop through each coin & if active, create pubkey consumer
  for (_key, value) in hashmap_clone.into_iter() {
    if *value == true {
      let mut group_id = &_key.to_string().to_lowercase();
      let mut topic: String = String::from(identity);
      topic.push_str("_");
      topic.push_str(&_key.to_string().to_lowercase());
      let env_key = &mut _key.to_string().to_owned();
      env_key.push_str("_PUB");
      initialize_consumer(
        kafka_config,
        &group_id,
        &topic,
        Some(env_key.to_string()),
        None,
        "general",
      );
    }
  }
}

// Initialize consumer used to read secure test messages from processors on secure partition
fn consume_processor_secure_test_message(
  kafka_config: &KafkaConfig,
  identity: &str,
  coin_hashmap: &HashMap<Coin, bool>,
) {
  let hashmap_clone = coin_hashmap.clone();

  // Loop through each coin & if active, create secure message consumer
  for (_key, value) in hashmap_clone.into_iter() {
    if *value == true {
      let mut group_id = &_key.to_string().to_lowercase();
      let mut topic: String = String::from(identity);
      topic.push_str("_");
      topic.push_str(&_key.to_string().to_lowercase());
      let env_key = &mut _key.to_string();
      // ENV_KEY references the processor pubkey we want to use with message box
      env_key.push_str("_PUB");
      initialize_consumer(
        kafka_config,
        &group_id,
        &topic,
        Some(env_key.to_string()),
        Some(&mut _key.to_string()),
        "secure",
      );
    }
  }
}

// Initializes consumer based on general or secure partition
fn initialize_consumer(
  kafka_config: &KafkaConfig,
  group_id: &str,
  topic: &str,
  env_key: Option<String>,
  coin: Option<&String>,
  partition_type: &str,
) {
  let consumer: BaseConsumer = ClientConfig::new()
    .set("bootstrap.servers", format!("{}:{}", kafka_config.host, kafka_config.port))
    .set("group.id", group_id)
    .set("auto.offset.reset", kafka_config.offset_reset.to_owned())
    .create()
    .expect("invalid consumer config");

  let mut env_key_ref: String = "".to_string();
  match env_key {
    Some(p) => {
      env_key_ref = String::from(p);
    }
    None => {}
  }

  let mut coin_ref: String = "".to_string();
  match coin {
    Some(p) => {
      coin_ref = String::from(p);
    }
    None => {}
  }

  match partition_type {
    "general" => {
      let mut tpl = rdkafka::topic_partition_list::TopicPartitionList::new();
      tpl.add_partition(&topic, 0);
      consumer.assign(&tpl).unwrap();

      tokio::spawn(async move {
        for msg_result in &consumer {
          let msg = msg_result.unwrap();
          let key: &str = msg.key_view().unwrap().unwrap();
          let msg_type = parse_message_type(&key);
          match msg_type {
            MessageType::ProcessorPubkeyToCoordinator => {
              let value = msg.payload().unwrap();
              let public_key = str::from_utf8(value).unwrap();
              info!("Received Pubkey from {}: {}", &key, &public_key);
              env::set_var(env_key_ref.clone(), public_key);
            }
            MessageType::ProcessorGeneralMessageToCoordinator => {
              let value = msg.payload().unwrap();
              let pub_msg = str::from_utf8(value).unwrap();
              info!("Received Public Message from {}", &key);
              info!("Public Message: {}", &pub_msg);
            }
            _ => {}
          }
        }
      });
    }
    "secure" => {
      let mut tpl = rdkafka::topic_partition_list::TopicPartitionList::new();
      tpl.add_partition(&topic, 1);
      consumer.assign(&tpl).unwrap();

      tokio::spawn(async move {
        for msg_result in &consumer {
          let msg = msg_result.unwrap();
          let key: &str = msg.key_view().unwrap().unwrap();
          let msg_type = parse_message_type(&key);
          match msg_type {
            MessageType::ProcessorSecureMessageToCoordinator => {
              let value = msg.payload().unwrap();
              // Creates Message box used for decryption
              let pubkey = message_box::PublicKey::from_trusted_str(
                &env::var(env_key_ref.to_string()).unwrap().to_string(),
              );

              let coord_priv =
                message_box::PrivateKey::from_string(env::var("COORD_PRIV").unwrap().to_string());

              let processor_id = retrieve_message_box_id(&coin_ref);

              let mut message_box_pubkeys = HashMap::new();
              message_box_pubkeys.insert(processor_id, pubkey);

              let message_box =
                MessageBox::new(message_box::ids::COORDINATOR, coord_priv, message_box_pubkeys);
              let encrypted_msg = str::from_utf8(value).unwrap();

              // Decrypt message using Message Box
              let encoded_string =
                message_box.decrypt_from_str(&processor_id, &encrypted_msg).unwrap();
              let decoded_string = String::from_utf8(encoded_string).unwrap();
              info!(
                "Received Encrypted Message from {}",
                &String::from(processor_id).to_lowercase()
              );
              info!("Decrypted Message: {}", &decoded_string);
            }
            _ => {}
          }
        }
      });
    }
    _ => {}
  }
}

// Initialize producer to send coordinator pubkey to processors on general partition
fn produce_coordinator_pubkey(
  kafka_config: &KafkaConfig,
  identity: &str,
  coin_hashmap: &HashMap<Coin, bool>,
) {
  let hashmap_clone = coin_hashmap.clone();

  info!("Sending Public Key");

  // Creates a producer to send coordinator pubkey message
  let producer: ThreadedProducer<_> = ClientConfig::new()
    .set("bootstrap.servers", format!("{}:{}", kafka_config.host, kafka_config.port))
    .create()
    .expect("invalid producer config");

  // Load Coordinator Pubkey
  let coord_pub = env::var("COORD_PUB");
  let msg = coord_pub.unwrap();

  for (_key, value) in hashmap_clone.into_iter() {
    if *value == true {
      // Sends message to Kafka
      producer
        .send(
          BaseRecord::to(&format!("{}_{}", &identity, &_key.to_string().to_lowercase()))
            .key(&format!("{}", MessageType::CoordinatorPubkeyToProcessor.to_string()))
            .payload(&msg)
            .partition(0),
        )
        .expect("failed to send message");
    }
  }
}

// Wait to receive all Processer Pubkeys
async fn process_received_pubkeys(coin_hashmap: &HashMap<Coin, bool>) {
  // Runs a loop to check if all processor keys are found
  let mut all_keys_found = false;
  while !all_keys_found {
    let hashmap_key_check = coin_hashmap.clone();
    let hashmap_clone = coin_hashmap.clone();

    let mut active_keys = 0;
    let mut keys_found = 0;
    for (_key, value) in hashmap_key_check.into_iter() {
      if *value == true {
        active_keys += 1;
      }
    }

    for (_key, value) in hashmap_clone.into_iter() {
      if *value == true {
        let env_key = &mut _key.to_string();
        env_key.push_str("_PUB");

        let pub_check = env::var(env_key);
        if !pub_check.is_err() {
          keys_found += 1;
        }
      }
    }

    if active_keys == keys_found {
      info!("All Processor Pubkeys Ready");
      all_keys_found = true;
    } else {
      // Add small delay for checking pubkeys
      tokio::time::sleep(Duration::from_millis(500)).await;
    }
  }
}

// Create Hashmap based on coins
fn create_coin_hashmap(chain_config: &ChainConfig) -> HashMap<Coin, bool> {
  let j = serde_json::to_string(&chain_config).unwrap();
  let mut coins: HashMap<Coin, bool> = HashMap::new();
  let coins_ref: HashMap<String, bool> = serde_json::from_str(&j).unwrap();
  for (key, value) in coins_ref.into_iter() {
    if value == true {
      match key.as_str() {
        "sri" => {
          coins.insert(Coin::SRI, true);
        }
        "btc" => {
          coins.insert(Coin::BTC, true);
        }
        "eth" => {
          coins.insert(Coin::ETH, true);
        }
        "xmr" => {
          coins.insert(Coin::XMR, true);
        }
        &_ => {}
      };
    }
  }
  coins
}

// Requests Coin ID from Message Box
fn retrieve_message_box_id(coin: &String) -> &'static str {
  let id = match coin.as_str() {
    "SRI" => message_box::ids::SRI_PROCESSOR,
    "BTC" => message_box::ids::BTC_PROCESSOR,
    "ETH" => message_box::ids::ETH_PROCESSOR,
    "XMR" => message_box::ids::XMR_PROCESSOR,
    &_ => "",
  };
  id
}

// Initialize a producer that sends a general & secure test message
async fn produce_general_and_secure_test_message(
  kafka_config: &KafkaConfig,
  identity: &str,
  coin_hashmap: &HashMap<Coin, bool>,
) {
  let hashmap_clone = coin_hashmap.clone();

  // Loop through each coin & if active, create general and secure producer
  for (key, value) in hashmap_clone.into_iter() {
    if *value == true {
      let mut topic: String = String::from(identity);
      topic.push_str("_");
      topic.push_str(&key.to_string().to_lowercase());
      let env_key = &mut key.to_string();
      env_key.push_str("_PUB");

      let processor_id = retrieve_message_box_id(&mut key.to_string());
      let mut msg: String = String::from("coordinator message to ");
      msg.push_str(&processor_id.to_lowercase());

      send_general_and_secure_test_message(
        &kafka_config,
        &topic,
        env_key.to_string(),
        &processor_id,
        msg.as_bytes().to_vec(),
      )
      .await;
    }
  }
}

// Initializes a producer then sends both a general and secure test message
async fn send_general_and_secure_test_message(
  kafka_config: &KafkaConfig,
  topic: &str,
  env_key: String,
  processor: &'static str,
  msg: Vec<u8>,
) {
  let producer: ThreadedProducer<_> = ClientConfig::new()
    .set("bootstrap.servers", format!("{}:{}", kafka_config.host, kafka_config.port))
    .create()
    .expect("invalid producer config");

  // Load Coordinator private key environment variable
  let coord_priv =
    message_box::PrivateKey::from_string(env::var("COORD_PRIV").unwrap().to_string());

  // Load Pubkeys for processors
  let pubkey =
    message_box::PublicKey::from_trusted_str(&env::var(env_key.to_string()).unwrap().to_string());

  let mut message_box_pubkey = HashMap::new();
  message_box_pubkey.insert(processor, pubkey);

  // Create Procesor Message Box
  let message_box = MessageBox::new(message_box::ids::COORDINATOR, coord_priv, message_box_pubkey);
  let enc = message_box.encrypt_to_string(&processor, &msg.clone());

  // Partition 0 is General
  producer
    .send(
      BaseRecord::to(&topic)
        .key(&format!("{}", MessageType::CoordinatorGeneralMessageToProcessor.to_string()))
        .payload(&msg)
        .partition(0),
    )
    .expect("failed to send message");
  // Add small delay for sending messages
  tokio::time::sleep(Duration::from_millis(500)).await;

  // Partition 1 is Secure
  producer
    .send(
      BaseRecord::to(&topic)
        .key(&format!("{}", MessageType::CoordinatorSecureMessageToProcessor.to_string()))
        .payload(&enc)
        .partition(1),
    )
    .expect("failed to send message");
  // Add small delay for sending messages
  tokio::time::sleep(Duration::from_millis(500)).await;
}
