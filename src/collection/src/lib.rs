use candid::{CandidType, Deserialize, Principal};
use ic_cdk;
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Serialize, Clone)]
struct Attribute {
    trait_type: String,
    value: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
struct Metadata {
    name: String,
    description: String,
    image: String,
    attributes: Vec<Attribute>,
    external_url: String,
}

#[derive(CandidType, Deserialize, Clone)]
struct CollectionInfo {
    owner: Principal,
    name: String,
    collection_size: u64,
    chain_name: String,
    description: String,
    standard: String,
}

thread_local! {
    static METADATA_STORE: RefCell<HashMap<u64, Metadata>> = RefCell::new(HashMap::new());
    static IMAGES: RefCell<HashMap<u64, Vec<u8>>> = RefCell::new(HashMap::new());
    static COLLECTION_INFO: RefCell<CollectionInfo> = RefCell::new(CollectionInfo {
        owner: Principal::anonymous(),
        name: String::new(),
        collection_size: 0,
        chain_name: String::new(),
        description: String::new(),
        standard: String::new(),
    });
}

//init
#[ic_cdk::init]
async fn init(info: CollectionInfo) {
    COLLECTION_INFO.with(|ci| *ci.borrow_mut() = info);
}

// modifiers
fn is_owner() -> Result<(), String> {
    // let caller = ic_cdk::caller();
    // COLLECTION_INFO.with(|info| {
    //     if info.borrow().owner == caller {
    //         Ok(())
    //     } else {
    //         Err("Only the owner can perform this action".to_string())
    //     }
    // })

    Ok(())
}

// Collection Info Getters
#[ic_cdk::query]
fn get_owner() -> Principal {
    COLLECTION_INFO.with(|info| info.borrow().owner)
}

#[ic_cdk::query]
fn get_name() -> String {
    COLLECTION_INFO.with(|info| info.borrow().name.clone())
}

#[ic_cdk::query]
fn get_collection_size() -> u64 {
    COLLECTION_INFO.with(|info| info.borrow().collection_size)
}

#[ic_cdk::query]
fn get_chain_name() -> String {
    COLLECTION_INFO.with(|info| info.borrow().chain_name.clone())
}

#[ic_cdk::query]
fn get_description() -> String {
    COLLECTION_INFO.with(|info| info.borrow().description.clone())
}

#[ic_cdk::query]
fn get_standard() -> String {
    COLLECTION_INFO.with(|info| info.borrow().standard.clone())
}

#[ic_cdk::query]
fn get_all_collection_info() -> CollectionInfo {
    COLLECTION_INFO.with(|info| info.borrow().clone())
}

// Collection Info Setters
#[ic_cdk::update]
fn set_owner(owner: Principal) -> Result<(), String> {
    is_owner()?;
    COLLECTION_INFO.with(|info| info.borrow_mut().owner = owner);
    Ok(())
}

#[ic_cdk::update]
fn set_name(name: String) -> Result<(), String> {
    is_owner()?;
    COLLECTION_INFO.with(|info| info.borrow_mut().name = name);
    Ok(())
}

#[ic_cdk::update]
fn set_collection_size(size: u64) -> Result<(), String> {
    is_owner()?;
    COLLECTION_INFO.with(|info| info.borrow_mut().collection_size = size);
    Ok(())
}

#[ic_cdk::update]
fn set_chain_name(name: String) -> Result<(), String> {
    is_owner()?;
    COLLECTION_INFO.with(|info| info.borrow_mut().chain_name = name);
    Ok(())
}

#[ic_cdk::update]
fn set_description(desc: String) -> Result<(), String> {
    is_owner()?;
    COLLECTION_INFO.with(|info| info.borrow_mut().description = desc);
    Ok(())
}

#[ic_cdk::update]
fn set_standard(std: String) -> Result<(), String> {
    is_owner()?;
    COLLECTION_INFO.with(|info| info.borrow_mut().standard = std);
    Ok(())
}

#[ic_cdk::update]
fn set_all_collection_info(info: CollectionInfo) -> Result<(), String> {
    is_owner()?;
    COLLECTION_INFO.with(|ci| *ci.borrow_mut() = info);
    Ok(())
}

// main functions
#[ic_cdk::update]
fn store_image(token_id: u64, image_data: Vec<u8>) -> Result<(), String> {
    IMAGES.with(|images| {
        images.borrow_mut().insert(token_id, image_data);
    });
    Ok(())
}

#[ic_cdk::query]
fn get_image(token_id: u64) -> Option<Vec<u8>> {
    IMAGES.with(|images| images.borrow().get(&token_id).cloned())
}

#[ic_cdk::query]
fn get_metadata(token_id: u64) -> Option<Metadata> {
    println!("hello");
    METADATA_STORE.with(|store| store.borrow().get(&token_id).cloned())
}

#[ic_cdk::update]
fn update_metadata(token_id: u64, new_metadata: Metadata) -> Result<(), String> {
    is_owner()?;

    METADATA_STORE.with(|store| {
        store.borrow_mut().insert(token_id, new_metadata);
    });

    Ok(())
}

#[ic_cdk::query]
fn get_all_metadata() -> Vec<(u64, Option<Metadata>)> {
    let collection_size = COLLECTION_INFO.with(|info| info.borrow().collection_size);
    let mut results = Vec::with_capacity(collection_size as usize);

    for token_id in 0..collection_size {
        let metadata = get_metadata(token_id);
        results.push((token_id, metadata));
    }

    results
}

#[ic_cdk::query]
fn http_request(request: HttpRequest) -> HttpResponse {
    let path: Vec<&str> = request.url.split("/").collect();
    ic_cdk::println!("Path :{:#?}", path);

    let metadata = get_metadata(0);

    HttpResponse {
        status_code: 200,
        headers: vec![("Content-Type".to_string(), "application/json".to_string())],
        body: serde_json::to_vec(&metadata).unwrap(),
    }

    // let path: Vec<&str> = request.url.split("/").collect();
    // ic_cdk::println!("Path :{:#?}", path);
    // if path.len() == 1 {
    //     if let Ok(token_id) = path[0].parse::<u64>() {
    //         if let Some(metadata) = get_metadata(token_id) {
    //             HttpResponse {
    //                 status_code: 200,
    //                 headers: vec![("Content-Type".to_string(), "application/json".to_string())],
    //                 body: serde_json::to_vec(&metadata).unwrap(),
    //             }
    //         } else {
    //             not_found()
    //         }
    //     } else {
    //         bad_request()
    //     }
    // } else {
    //     not_found()
    // }
}

#[derive(CandidType, Deserialize)]
struct HttpRequest {
    url: String,
    method: String,
    body: Vec<u8>,
    headers: Vec<(String, String)>,
}

#[derive(CandidType, Serialize)]
struct HttpResponse {
    status_code: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

fn not_found() -> HttpResponse {
    HttpResponse {
        status_code: 404,
        headers: vec![],
        body: b"Not Found".to_vec(),
    }
}

fn bad_request() -> HttpResponse {
    HttpResponse {
        status_code: 400,
        headers: vec![],
        body: b"Bad Request".to_vec(),
    }
}

ic_cdk::export_candid!();
