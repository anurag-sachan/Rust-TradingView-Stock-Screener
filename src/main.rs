use reqwest::Client;
use serde_json::Value;
use tokio;
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let mut symbols_list: Vec<String> = Vec::new();

    let scan_list_url = "https://scanner.tradingview.com/india/scan";
    let body = r#"{
        "columns": [
            "name", "description", "logoid", "update_mode", "type", "typespecs", "close",
            "pricescale", "minmov", "fractional", "minmove2", "currency", "change", "volume",
            "relative_volume_10d_calc", "market_cap_basic", "fundamental_currency_code",
            "price_earnings_ttm", "earnings_per_share_diluted_ttm",
            "earnings_per_share_diluted_yoy_growth_ttm", "dividends_yield_current",
            "sector.tr", "market", "sector", "recommendation_mark", "exchange", "CCI20"
        ],
        "filter": [
            {
                "left": "CCI20",
                "operation": "in_range",
                "right": [30, 90]
            }
        ],
        "ignore_unknown_fields": false,
        "options": {"lang": "en"},
        "range": [0, 800],
        "sort": {"sortBy": "market_cap_basic", "sortOrder": "desc"},
        "symbols": {},
        "markets": ["india"],
        "filter2": {
            "operator": "and",
            "operands": [
                {
                    "operation": {
                        "operator": "or",
                        "operands": [
                            {
                                "operation": {
                                    "operator": "and",
                                    "operands": [
                                        {"expression": {"left": "type", "operation": "equal", "right": "stock"}},
                                        {"expression": {"left": "typespecs", "operation": "has", "right": ["common"]}}
                                    ]
                                }
                            },
                            {
                                "operation": {
                                    "operator": "and",
                                    "operands": [
                                        {"expression": {"left": "type", "operation": "equal", "right": "stock"}},
                                        {"expression": {"left": "typespecs", "operation": "has", "right": ["preferred"]}}
                                    ]
                                }
                            },
                            {"operation": {"operator": "and", "operands": [{"expression": {"left": "type", "operation": "equal", "right": "dr"}}]}},
                            {
                                "operation": {
                                    "operator": "and",
                                    "operands": [
                                        {"expression": {"left": "type", "operation": "equal", "right": "fund"}},
                                        {"expression": {"left": "typespecs", "operation": "has_none_of", "right": ["etf"]}}
                                    ]
                                }
                            }
                        ]
                    }
                }
            ]
        }
    }"#;

    let response = client
        .post(scan_list_url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    let response_json = response.json::<Value>().await?;

    let symbols = response_json["data"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|item| {
            if let Some(symbol) = item["s"].as_str() {
                if symbol.starts_with("NSE:") {
                    return Some(symbol.to_string());
                }
            }
            None
        })
        .collect::<Vec<String>>();

    symbols_list.extend(symbols.iter().cloned());

    // let body = r#"{
    //     "columns": [
    //         "name", "description", "logoid", "update_mode", "type", "typespecs", "close",
    //         "pricescale", "minmov", "fractional", "minmove2", "currency", "change", "volume",
    //         "relative_volume_10d_calc", "market_cap_basic", "fundamental_currency_code",
    //         "price_earnings_ttm", "earnings_per_share_diluted_ttm",
    //         "earnings_per_share_diluted_yoy_growth_ttm", "dividends_yield_current",
    //         "sector.tr", "market", "sector", "recommendation_mark", "exchange", "CCI20"
    //     ],
    //     "filter": [
    //         {
    //             "left": "CCI20",
    //             "operation": "in_range",
    //             "right": [-5, 5]
    //         }
    //     ],
    //     "ignore_unknown_fields": false,
    //     "options": {"lang": "en"},
    //     "range": [0, 800],
    //     "sort": {"sortBy": "market_cap_basic", "sortOrder": "desc"},
    //     "symbols": {},
    //     "markets": ["india"],
    //     "filter2": {
    //         "operator": "and",
    //         "operands": [
    //             {
    //                 "operation": {
    //                     "operator": "or",
    //                     "operands": [
    //                         {
    //                             "operation": {
    //                                 "operator": "and",
    //                                 "operands": [
    //                                     {"expression": {"left": "type", "operation": "equal", "right": "stock"}},
    //                                     {"expression": {"left": "typespecs", "operation": "has", "right": ["common"]}}
    //                                 ]
    //                             }
    //                         },
    //                         {
    //                             "operation": {
    //                                 "operator": "and",
    //                                 "operands": [
    //                                     {"expression": {"left": "type", "operation": "equal", "right": "stock"}},
    //                                     {"expression": {"left": "typespecs", "operation": "has", "right": ["preferred"]}}
    //                                 ]
    //                             }
    //                         },
    //                         {"operation": {"operator": "and", "operands": [{"expression": {"left": "type", "operation": "equal", "right": "dr"}}]}},
    //                         {
    //                             "operation": {
    //                                 "operator": "and",
    //                                 "operands": [
    //                                     {"expression": {"left": "type", "operation": "equal", "right": "fund"}},
    //                                     {"expression": {"left": "typespecs", "operation": "has_none_of", "right": ["etf"]}}
    //                                 ]
    //                             }
    //                         }
    //                     ]
    //                 }
    //             }
    //         ]
    //     }
    // }"#;

    // let response = client
    //     .post(scan_list_url)
    //     .header("Content-Type", "application/json")
    //     .body(body)
    //     .send()
    //     .await?;

    // let response_json = response.json::<Value>().await?;

    // let symbols = response_json["data"]
    //     .as_array()
    //     .unwrap_or(&vec![])
    //     .iter()
    //     .filter_map(|item| {
    //         if let Some(symbol) = item["s"].as_str() {
    //             if symbol.starts_with("NSE:") {
    //                 return Some(symbol.to_string());
    //             }
    //         }
    //         None
    //     })
    //     .collect::<Vec<String>>();

    // symbols_list.extend(symbols.iter().cloned());

    // let body = r#"{
    //     "columns": [
    //         "name", "description", "logoid", "update_mode", "type", "typespecs", "close",
    //         "pricescale", "minmov", "fractional", "minmove2", "currency", "change", "volume",
    //         "relative_volume_10d_calc", "market_cap_basic", "fundamental_currency_code",
    //         "price_earnings_ttm", "earnings_per_share_diluted_ttm",
    //         "earnings_per_share_diluted_yoy_growth_ttm", "dividends_yield_current",
    //         "sector.tr", "market", "sector", "recommendation_mark", "exchange", "CCI20"
    //     ],
    //     "filter": [
    //         {
    //             "left": "CCI20",
    //             "operation": "in_range",
    //             "right": [90, 110]
    //         }
    //     ],
    //     "ignore_unknown_fields": false,
    //     "options": {"lang": "en"},
    //     "range": [0, 800],
    //     "sort": {"sortBy": "market_cap_basic", "sortOrder": "desc"},
    //     "symbols": {},
    //     "markets": ["india"],
    //     "filter2": {
    //         "operator": "and",
    //         "operands": [
    //             {
    //                 "operation": {
    //                     "operator": "or",
    //                     "operands": [
    //                         {
    //                             "operation": {
    //                                 "operator": "and",
    //                                 "operands": [
    //                                     {"expression": {"left": "type", "operation": "equal", "right": "stock"}},
    //                                     {"expression": {"left": "typespecs", "operation": "has", "right": ["common"]}}
    //                                 ]
    //                             }
    //                         },
    //                         {
    //                             "operation": {
    //                                 "operator": "and",
    //                                 "operands": [
    //                                     {"expression": {"left": "type", "operation": "equal", "right": "stock"}},
    //                                     {"expression": {"left": "typespecs", "operation": "has", "right": ["preferred"]}}
    //                                 ]
    //                             }
    //                         },
    //                         {"operation": {"operator": "and", "operands": [{"expression": {"left": "type", "operation": "equal", "right": "dr"}}]}},
    //                         {
    //                             "operation": {
    //                                 "operator": "and",
    //                                 "operands": [
    //                                     {"expression": {"left": "type", "operation": "equal", "right": "fund"}},
    //                                     {"expression": {"left": "typespecs", "operation": "has_none_of", "right": ["etf"]}}
    //                                 ]
    //                             }
    //                         }
    //                     ]
    //                 }
    //             }
    //         ]
    //     }
    // }"#;

    // let response = client
    //     .post(scan_list_url)
    //     .header("Content-Type", "application/json")
    //     .body(body)
    //     .send()
    //     .await?;

    // let response_json = response.json::<Value>().await?;

    // let symbols = response_json["data"]
    //     .as_array()
    //     .unwrap_or(&vec![])
    //     .iter()
    //     .filter_map(|item| {
    //         if let Some(symbol) = item["s"].as_str() {
    //             if symbol.starts_with("NSE:") {
    //                 return Some(symbol.to_string());
    //             }
    //         }
    //         None
    //     })
    //     .collect::<Vec<String>>();

    // symbols_list.extend(symbols.iter().cloned());
    // symbols_list.extend(symbols);

    // let mut symbols_cci_range: HashMap<i32, String> = HashMap::new();
    let mut symbols_cci_range: HashSet<String> = HashSet::new();
    let mut var: i32=0;
    // for symbol in &symbols_list {
    //     val=val+1;
    //     println!("{}", symbol);
    // }
    // println!("val: {}",val);

    for symbol in &symbols_list {
        var=var+1;
        let url = format!("https://scanner.tradingview.com/symbol?symbol={}&fields=CCI20|15&no_404=true", symbol);
        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            let resp_json: Value = response.json().await?;
            // println!("{}",resp_json);
            if let Some(cci_value) = resp_json.get("CCI20|15").and_then(|v| v.as_f64()) {
                if (30.0..=90.0).contains(&cci_value) {
                // if (-10.0..=5.0).contains(&cci_value) || (-130.0..=-90.0).contains(&cci_value) || (90.0..=110.0).contains(&cci_value) {
                    // let range_msg = format!("CCI20: {}", cci_value);
                    // symbols_cci_range.insert(symbol.to_string(), range_msg);
                    // symbols_cci_range.insert(var, symbol.to_string());
                    symbols_cci_range.insert(symbol.to_string());
                }
            } else {
                // symbols_cci_range.insert(var, symbol.to_string());
                symbols_cci_range.insert(symbol.to_string());
            }
            // if let Some(cci_value) = resp_json.get("CCI20|15").and_then(|v| v.as_f64()) {
            //     let range_msg = if (-5.0..=5.0).contains(&cci_value) || (-110.0..=-90.0).contains(&cci_value) || (90.0..=110.0).contains(&cci_value) {
            //         cci += 1;
            //         format!("CCI20: {} ✅", cci_value)
            //     } else {
            //         format!("❌")
            //     };
            //     symbols_cci_range.insert(symbol.to_string(), range_msg);
            // } else {
            //     non_cci += 1;
            //     symbols_cci_range.insert(symbol.to_string(), "CCI20 value not found".to_string());
            // }
        } else {
            symbols_cci_range.insert(symbol.to_string());
        }
    }
    // println!("total scans: {}, w/ cci: {}, without cci: {}", val, cci, non_cci);

    // for symbol in &symbols_list {
    //     println!("{}", symbol);
    // }
    // println!("Total symbols: {}", symbols_list.len());

    // println!("CCI20 Range Results:");
    let mut val=0;
    for symbol in &symbols_cci_range {
        val=val+1;
        println!("{}, {}", val, symbol);
    }
    println!();
    println!("TOTAL: {}", val);
    Ok(())
}

