use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use itertools::Itertools;

pub fn run() -> Result<(), Error> {
    println!("Running...");
    let binary_string = parse_data();
    println!("version total {:?}", parse_packet(&binary_string));

    Ok(())
}

fn parse_packet(binary_string: &String) -> (i64, String) {
    println!("Payload string {}", binary_string);

    if binary_string == "" {
        return (0, "".to_string());
    }

    let (version, type_id, payload_str) = extract_packet_header(&binary_string);

    if type_id == 4 {
        println!("Literal subpacket");
        let (payload_value, remaining) = extract_literal_payload(&payload_str);
        // println!("version {}, type_id {}, payload {} ", version, type_id, payload_value);
        return (version, remaining);
    } else if payload_str.chars().nth(0).unwrap() == '0' {
        let subpacket_length = i64::from_str_radix(&payload_str[1..16], 2).unwrap();
        println!("Operator packet 1");
        let mut length_read = 0 as i64;
        let mut subpacket = payload_str[16..].to_string();
        let mut local_total = 0;
        while length_read != subpacket_length && subpacket != "" && !subpacket.chars().all(|c| c == '0'){
            // println!("length read {}", length_read);
            let (v, remaining) = parse_packet(&subpacket);
            length_read += remaining.len() as i64;
            subpacket = remaining;
            local_total+=v;
        }
        // println!("Operator and literal subpackets {}, ", &payload_str[16..(subpacket_length + 16) as usize]);
        return (version + local_total, subpacket);
    } else if payload_str.chars().nth(0).unwrap() == '1' {
        println!("Operator packet 2");

        let subpacket_count = i64::from_str_radix(&payload_str[1..12], 2).unwrap();
        println!("Count {}, ", subpacket_count);

        let mut subpacket = payload_str[12..].to_string();
        let mut local_total = 0;
        for i in 0..subpacket_count {
            let (v, remaining) = parse_packet(&subpacket);
            subpacket = remaining;
            local_total+=v;
        }

        return (version + local_total, subpacket);
    }

    (0, "".to_string())
}

fn extract_packet_header(binary_string: &&String) -> (i64, i64, String) {
    let version = i64::from_str_radix(&binary_string[0..3], 2).unwrap();
    let type_id = i64::from_str_radix(&binary_string[3..6], 2).unwrap();
    (version, type_id, binary_string[6..binary_string.len()].to_string())
}

fn extract_literal_payload(payload_string: &String) -> (i64, String) {
    let payload_vec = payload_string.chars().collect::<Vec<char>>();

    let mut payloads = vec![];
    let mut done = false;
    let mut payload_length = 0;

    for chunk in payload_vec.chunks(5) {
        let is_all_zeroes = chunk.iter().all(|&c| c == '0');

        if chunk[0] == '1' || chunk[0] == '0' {
            payloads.push(chunk.iter().dropping(1).join(""));
            payload_length += 5;
        }

        if chunk[0] == '0' { break; }
    }

    println!("left {:?}, {}", payloads, payload_string);
    let value= i64::from_str_radix(&payloads.iter().join(""), 2).unwrap();

    if payload_length >= payload_string.len() {
        // println!("got here");
        return (value, "".to_string());
    }

    return (value , payload_string[payload_length..].to_string())
}


fn parse_literal_payload(binary_string: &str) {
    let version = i64::from_str_radix(&binary_string[0..3], 2).unwrap();
    let type_id = i64::from_str_radix(&binary_string[3..6], 2).unwrap();
    let trailing_zeros = i64::from_str_radix(&binary_string, 2).unwrap().trailing_zeros();

    let payload = &binary_string[6..binary_string.len() - trailing_zeros as usize];
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

    let hex = include_str!("../test_inputs/example_16.txt");
    let binary_string = hex.chars().map(|c| hex_dict[&c]).join("");
    binary_string
}




