mod pb;

use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{self as eth};
use pb::sf::substreams::index::v1::Keys;
substreams_ethereum::init!();

#[substreams::handlers::map]
fn block_index(blk: eth::Block) -> Result<Keys, substreams::errors::Error> { 
    let mut keys = Keys::default();

    blk.logs().into_iter().for_each(|log| {
        if log.topics().len() > 0 {
            let k_log_sign = format!("es:{}", Hex::encode(log.topics().get(0).unwrap()));
            keys.keys.push(k_log_sign);
        }
        
        let k_log_address = format!("ea:{}", Hex::encode(log.address()));

        keys.keys.push(k_log_address);
    });

    blk.calls().into_iter().for_each(|call| {
        let addr_bytes = &call.call.address;

        let k_call_address = format!("ca:{}", Hex::encode(addr_bytes));
        keys.keys.push(k_call_address);

        let input_bytes = &call.call.input;
        
        if input_bytes.len() >= 4{
            let k_call_method = format!("cm:{}", Hex::encode(&input_bytes[..4]));
            keys.keys.push(k_call_method);
        }
    });
    
    Ok(keys)
}
