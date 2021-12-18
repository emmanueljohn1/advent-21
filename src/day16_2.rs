use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::u128;
use itertools::{Itertools};

pub fn run() -> Result<(), Error> {
    println!("Running...");
    let binary_string = parse_data();
    let (value, _) = parse_packet(&binary_string);
    println!("Value = {}", value);

    Ok(())
}

fn parse_packet(binary_string: &String) -> (u128, String) {
    let (version, type_id, payload_str) = extract_packet_header(binary_string);

    if type_id != 4 { // operator packet
        return if payload_str.chars().nth(0).unwrap() == '0' {
            process_subpacket_length_type(&type_id, &payload_str)
        } else {
            process_subpacket_count_type(&type_id, &payload_str)
        }
    }
    // Literal packet
    let (payload_value, remaining_string) = extract_literal_payload(&payload_str);
    return (payload_value, remaining_string);
}

fn process_subpacket_count_type(type_id: &i64, payload_str: &String) -> (u128, String) {
    let subpacket_count = u128::from_str_radix(&payload_str[1..12], 2).unwrap();
    let mut subpacket = payload_str[12..].to_string();

    //first value
    let (value1, remaining_string) = parse_packet(&subpacket);
    subpacket = remaining_string.to_string();

    // get the rest
    let mut read_count = 1;
    let mut total_value = value1;

    while read_count != subpacket_count {
        if subpacket.is_empty() || subpacket.len() < 6 {
            break;
        }
        let (value2, remaining_string) = parse_packet(&subpacket);
        read_count += 1;
        subpacket = remaining_string;
        total_value = op(&type_id, total_value, value2);
    }

    (total_value, subpacket)
}

fn process_subpacket_length_type(type_id: &i64, payload_str: &String) -> (u128, String) {
    let subpacket_length = u128::from_str_radix(&payload_str[1..16], 2).unwrap();
    let mut subpacket = payload_str[16..17 + subpacket_length as usize].to_string();
    //first value
    let (value1, remaining_string) = parse_packet(&subpacket);
    subpacket = remaining_string.to_string();

    // get the rest
    let mut length_read = subpacket.len() - remaining_string.len();
    let mut total_value = value1;

    while length_read != subpacket_length.try_into().unwrap() {
        if subpacket.is_empty() || subpacket.len() < 6 {
            break;
        }
        let (value2, remaining_string) = parse_packet(&subpacket);
        length_read += subpacket.len() - remaining_string.len();
        subpacket = remaining_string;
        total_value = op(&type_id, total_value, value2);
    }

    (total_value, payload_str[(16 + subpacket_length) as usize..].to_string())
}

fn extract_packet_header(binary_string: &String) -> (i64, i64, String) {
    let version = i64::from_str_radix(&binary_string[0..3], 2).unwrap();
    let type_id = i64::from_str_radix(&binary_string[3..6], 2).unwrap();
    (version, type_id, binary_string[6..binary_string.len()].to_string())
}

fn extract_literal_payload(payload_string: &String) -> (u128, String) {
    let payload_vec = payload_string.chars().collect::<Vec<char>>();

    let mut payloads = String::from("");
    let mut original_string = String::from("");

    for chunk in payload_vec.chunks(5) {
        original_string.push_str(&*String::from(chunk.iter().join("")));

        if chunk.len() < 5 {
            break;
        }

        if chunk[0] == '1' || chunk[0] == '0' { // first time we are parsing for payload value
            payloads.push_str(chunk.iter().dropping(1).join("").as_str());
        }

        if chunk[0] == '0' { break; }
    }

    let value = i64::from_str_radix(&payloads, 2).unwrap();
    if original_string.len() >= payload_string.len() {
        return (value as u128, "".to_string());
    }
    return (value as u128, payload_string[original_string.len()..].to_string());
}

// Had to use u128 because multiplication was overflowing
fn op(type_id: &i64, val1: u128, val2: u128) -> u128 {
    match type_id {
        0 => val1 + val2,
        1 => val1 * val2,
        2 => min(val1, val2),
        3 => max(val1, val2),
        5 => {
            if val1 > val2 { 1 } else { 0 }
        }
        6 => {
            if val1 < val2 {
                1
            } else {
                0
            }
        }
        7 => {
            if val1 == val2 { 1 } else { 0 }
        }
        _ => {
            println!("unknown - This should not happen");
            0
        }
    }
}

fn parse_literal_payload(binary_string: &str) {
    let version = i128::from_str_radix(&binary_string[0..3], 2).unwrap();
    let type_id = i128::from_str_radix(&binary_string[3..6], 2).unwrap();
    let payload = &binary_string[6..binary_string.len()];
}

fn parse_data() -> String {
    let hex_dict = HashMap::from([('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111")]);

    let hex = include_str!("../test_inputs/input_16.txt");
    let binary_string = hex.chars().map(|c| hex_dict[&c]).join("");
    binary_string
}




