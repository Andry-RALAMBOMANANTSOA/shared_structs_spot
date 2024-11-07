use serde::{Serialize, Deserialize};
use std::{fmt,collections::{BTreeMap,HashMap}, hash::Hash};


#[derive(Debug, Clone, PartialEq,Serialize,Deserialize)]
 pub enum OrderSide {
     Buy,
     Sell,
     Unspecified,
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderSide::Buy => write!(f, "Buy"),
            OrderSide::Sell => write!(f, "Sell"),
            OrderSide::Unspecified => write!(f, ""),
        }
    }
}

#[derive(Debug, Clone, PartialEq,Serialize,Deserialize)]
 pub enum OrderExpiration {
    FOK,  // Fill or Kill
    IOC,  // Immediate or Cancel
    GTC,  // Good Till Cancelled
    GTD,// Good Till Date
    Unspecified,  
}
impl fmt::Display for OrderExpiration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderExpiration::FOK => write!(f, "FOK"),
            OrderExpiration::IOC => write!(f, "IOC"),
            OrderExpiration::GTC => write!(f, "GTC"),
            OrderExpiration::GTD => write!(f, "GTD"),
            OrderExpiration::Unspecified => write!(f, ""),
        }
    }
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct IcebergOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub total_quantity:f32,
    pub visible_quantity:f32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration, //tsisy raha market
    pub price: i32, //tsisy raha market
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct IcebergOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub unix_time:i64,
    pub iceberg_identifier:i64,
    pub total_quantity:f32,
    pub visible_quantity:f32,
    pub resting_quantity:f32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration, //tsisy raha market
    pub price: i32, //tsisy raha market
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct LimitOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub order_identifier: Option<i64>,
    pub order_quantity:f32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration, //tsisy raha market
    pub price: i32, //tsisy raha market
    
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MarketOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub order_identifier: Option<i64>,
    pub order_quantity:f32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
  
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct StopOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub order_quantity:f32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub trigger_price: i32, 
  
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct StopLimitOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub order_quantity:f32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration, 
    pub trigger_price: i32, 
    pub price:i32,
  
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ModifyOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub order_identifier: i64, //misy raha avy amin'ny modify,stop,mit,sns
    pub new_quantity:f32,
    
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ModifyIcebergOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub iceberg_identifier: i64, //misy raha avy amin'ny modify,stop,mit,sns
    pub new_quantity:f32,
    pub new_visible_quantity:f32,
    
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct DeleteOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub order_identifier: i64, //misy raha avy amin'ny modify,stop,mit,sns
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct DeleteIcebergOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub iceberg_identifier: i64, 
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct TraderOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub order_quantity: f32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub price: i32,
    
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct TraderStopOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub order_quantity: f32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub trigger_price: i32,
  
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct TraderStopLimitOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub order_quantity: f32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub trigger_price: i32,
    pub price: i32,
  
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct DeletedOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub order_quantity: f32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub price: i32,
 
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct DeletedIcebergStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub iceberg_identifier: i64,
    pub total_quantity: f32,
    pub visible_quantity:f32,
    pub resting_quantity:f32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub price: i32,
   
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct DeletedStopOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub order_quantity: f32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub trigger_price: i32,
  
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct DeletedStopLimitOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub order_quantity: f32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub trigger_price: i32,
    pub price :i32,
  
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ModifiedOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub older_order_quantity: f32,
    pub new_order_quantity: f32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub price: i32,
   
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ModifiedIcebergStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub iceberg_identifier: i64,
    pub older_quantity: f32,
    pub new_quantity: f32,
    pub older_visible_quantity: f32,
    pub new_visible_quantity: f32,
    pub older_resting_quantity:f32,
    pub new_resting_quantity:f32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub price: i32,
   
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ModifiedStopOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub older_order_quantity: f32,
    pub new_order_quantity: f32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub trigger_price: i32,
 
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ModifiedStopLimitOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub older_order_quantity: f32,
    pub new_order_quantity: f32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub trigger_price: i32,
    pub price: i32,
  
}


#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MatchStruct {
    pub market: String,
    pub broker_identifier_taker: String,
    pub broker_identifier_maker: String,
    pub unix_time: i64,
    pub match_identifier:i64,
    pub trader_identifier_taker: i64,
    pub order_identifier_taker: i64,
    pub trader_identifier_maker: i64,
    pub order_identifier_maker: i64,
    pub maker_order_side : OrderSide,
    pub taker_order_side : OrderSide,
    pub taker_type:String,
    pub expiration_taker:OrderExpiration,
    pub expiration_maker:OrderExpiration,
    pub order_quantity: f32,
    pub order_side: OrderSide,
    pub price: i32,
 
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct PostTraderInf {
    pub unix_time:i64,
    pub market: String,
    pub broker_identifier: String,
    pub match_identifier:i64,
    pub trader_identifier:i64,
    pub order_identifier:i64,
    pub trader_identifier_matcher:i64,
    pub order_identifier_matcher:i64,
    pub order_quantity:f32,
    pub order_side: OrderSide,
    pub asset_a_calcbalance:f32,
    pub asset_b_calcbalance:f32,
  
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct BBO {
   
    pub unix_time: i64,
    pub market:String,
    pub ask_price: Option<i32>,
    pub bid_price:Option<i32>,
    pub ask_size:Option<f32>,
    pub bid_size:Option<f32>,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct TimeSale {
    
    pub market: String,
    pub exchange:String,
    pub unix_time: i64,
    pub order_quantity: f32,
    pub order_side: OrderSide,
    pub price:i32,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Last {
    pub unix_time:i64,
    pub market:String,
    pub price:i32,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct OHLCV {
    pub state:String, //progessiong, completed
    pub datetime:i64,
    pub timeframe:String,
    pub market:String,
    pub open: i32,
    pub high: i32,
    pub low: i32,
    pub close: i32,
    pub buy_volume: f32,
    pub sell_volume: f32,
    pub differential_volume: f32,
    pub total_volume: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VolumeData {
    pub buy_volume: f32,
    pub sell_volume: f32,
    pub differential_volume: f32, // buy_volume - sell_volume
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct VP {
    pub state:String, //progessiong, completed
    pub datetime:i64,
    pub timeframe:String,
    pub market:String,
    pub volume_points: BTreeMap<i32, VolumeData>, // i32 is the price point
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MBPData {
   
    pub mbp: BTreeMap<i32, f32>,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct OB {
    pub market:String,
    pub bid: BTreeMap<i32, f32>,
    pub ask: BTreeMap<i32, f32>,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct PriceLevel {
    pub price: i32,
    pub quantity: f32,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct FullOB {
    pub unix_time: i64,
    pub market:String,
    pub bid: Vec<PriceLevel>,//MBPData
    pub ask: Vec<PriceLevel>,//MBPData
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MBOData {
  
    pub mbo: BTreeMap<i32, Vec<f32>>,//key is price and value is quantity
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Volume {
   
    pub unix_time: i64,
    pub market:String,
    pub buy_volume:f32,
    pub sell_volume:f32,
    pub price:i32,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MAPData {
    
  
    pub map:BTreeMap<i32, Vec<i64>>,//key is price, value is vec of order identifier
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MAPStopData {
  
    pub map:BTreeMap<i32, Vec<i64>>,//key is price, value is vec of order identifier
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MAPStopLimitData {
    pub map:BTreeMap<i32, Vec<i64>>,//key is price, value is vec of order identifier
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Messaging {
    pub unix_time : i64,
    pub market : String,
    pub broker_identifier : String,
    pub trader_identifier : i64,
    pub message : String,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MBPEvents {
    pub unix_time : i64,
    pub market:String,
    pub side : String,
    pub event_value:f32,
    pub event_price:i32,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ExecutedStop {
    pub unix_time : i64,
    pub market : String,
    pub broker_identifier : String,
    pub trader_identifier : i64,
    pub order_identifier : i64,
    pub order_quantity: f32,
    pub order_side : OrderSide,
    pub expiration : OrderExpiration,
    pub trigger_price : i32,
  
       
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ExecutedStopLimit {
    pub unix_time : i64,
    pub market : String,
    pub broker_identifier : String,
    pub trader_identifier : i64,
    pub order_identifier : i64,
    pub order_quantity: f32,
    pub order_side : OrderSide,
    pub expiration : OrderExpiration,
    pub trigger_price : i32,
    pub price : i32,
  

}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ExecutedIceberg {
    pub unix_time : i64,
    pub market : String,
    pub broker_identifier : String,
    pub trader_identifier : i64,
    pub iceberg_identifier : i64,
    pub executed_quantity: f32,
    pub order_side : OrderSide,
    pub expiration : OrderExpiration,
    pub price : i32,
   
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct StructData {
    pub market:String,
    pub data_struct: HashMap<i64, TraderOrderStruct>,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct StructStopData {
    pub market:String,
   pub data_struct: HashMap<i64, TraderStopOrderStruct>,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct StructStopLimitData {
    pub market:String,
   pub data_struct: HashMap<i64, TraderStopLimitOrderStruct>,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Save {
    pub market:String,
   
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub enum Structs {
    IcebergOrder(IcebergOrder),
    LimitOrder(LimitOrder),
    MarketOrder(MarketOrder),
    StopOrder(StopOrder),
    StopLimitOrder(StopLimitOrder),
    ModifyOrder(ModifyOrder),
    ModifyIcebergOrder(ModifyIcebergOrder),
    DeleteOrder(DeleteOrder),
    DeleteIcebergOrder(DeleteIcebergOrder),
    IcebergOrderStruct(IcebergOrderStruct),
    TraderOrderStruct(TraderOrderStruct),
    TraderStopOrderStruct(TraderStopOrderStruct),
    TraderStopLimitOrderStruct(TraderStopLimitOrderStruct),
    DeletedOrderStruct(DeletedOrderStruct),
    DeletedIcebergStruct(DeletedIcebergStruct),
    DeletedStopOrderStruct(DeletedStopOrderStruct),
    DeletedStopLimitOrderStruct(DeletedStopLimitOrderStruct),
    ModifiedStopOrderStruct(ModifiedStopOrderStruct),
    ModifiedStopLimitOrderStruct(ModifiedStopLimitOrderStruct),
    ModifiedOrderStruct(ModifiedOrderStruct),
    ModifiedIcebergStruct(ModifiedIcebergStruct),
    ExecutedStop(ExecutedStop),
    ExecutedStopLimit(ExecutedStopLimit),
    ExecutedIceberg(ExecutedIceberg),
    Messaging(Messaging),
    MatchStruct(MatchStruct),
    PostTraderInf(PostTraderInf),
    BBO(BBO),
    TimeSale(TimeSale),
    Last(Last),
    MBPData(MBPData),
    MBOData(MBOData),
    Volume(Volume),
    MAPData(MAPData),
    MAPStopData(MAPStopData),
    MAPStopLimitData(MAPStopLimitData),
    MBPEvents(MBPEvents),
    MarketConf(MarketConf),
    StructData(StructData),
    StructStopData(StructStopData),
    StructStopLimitData(StructStopLimitData),
    OB(OB),
    FullOB(FullOB),
  
    Save(Save),
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MarketConfig {
    pub market_list: Vec<String>,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct BrokerConfig {
    pub broker_list: Vec<String>, // Broker identifier -> HMAC key
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarketSpec {
    pub tick_size: i32,
    pub min_quantity: f32,
    pub max_quantity: f32,
    pub iceberg_min_quantity:f32,
   
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarketSpecConfig {
    pub market_specification: HashMap<String, MarketSpec>,
}
#[derive(Serialize)]
pub struct MSResponse {
    pub message: String,
    pub success: bool,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct BrokerConfigDedicaced {
    pub broker_list: HashMap<String, String>, // Broker identifier -> HMAC key
}
//Broker level
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct PaidCommission {
    pub unix_time: i64, 
    pub asset_name:String,
    pub broker_identifier:String,
    pub trader_identifier:i64,
    pub commission_amount:f32,
  
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct TraderBalance {
    pub asset_name: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub asset_balance:f32,
  
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct NumberToFetch {
    
    pub number:u32,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MarketConf {
    pub exchange: String,
    pub market_name: String,
    pub asset_a_name: String,
    pub asset_b_name: String,
}

