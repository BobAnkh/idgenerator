use crate::idgen::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

pub struct IdHelper;

/// one instance with one generator (should be given a unique worker id)
static mut ID_GEN_INSTANCE: Option<Arc<Mutex<DefaultIdGenerator>>> = None;

impl IdHelper {
    /// One generator in instance only
    pub fn init() {
        unsafe {
            ID_GEN_INSTANCE = Some(Arc::new(Mutex::new(DefaultIdGenerator::default())));
        }
    }

    fn id_gen_instance() -> Arc<Mutex<DefaultIdGenerator>> {
        unsafe { ID_GEN_INSTANCE.as_ref().unwrap().clone() }
    }

    /// Set options
    pub fn set_id_generator(options: IdGeneratorOptions) {
        let idgen_arc = IdHelper::id_gen_instance();
        let mut idgen = idgen_arc.lock().unwrap();
        idgen.worker.set_options(options);
    }

    /// Set worker id of the only generator in instance
    pub fn set_worker_id(worker_id: u32) {
        let idgen_arc = IdHelper::id_gen_instance();
        let mut idgen = idgen_arc.lock().unwrap();
        let options = IdGeneratorOptions::new(worker_id);
        idgen.worker.set_options(options);
    }

    /// Get a unique id
    pub fn next_id() -> i64 {
        let idgen_arc = IdHelper::id_gen_instance();
        let mut idgen = idgen_arc.lock().unwrap();
        idgen.worker.next_id()
    }
}

// lazy_static! {
//     static ref ID_GEN_INSTANCE_VEC: Option<Vec<Arc<Mutex<DefaultIdGenerator>>>> =
//         None;
// }

/// multiple generators with different worker id in one instance, using `Vector`
static mut ID_GEN_INSTANCE_VEC: Option<Vec<Arc<Mutex<DefaultIdGenerator>>>> = None;

static mut ID_BASE: u32 = 1;
static mut INTERVAL: u32 = 1;

pub struct IdVecHelper;
impl IdVecHelper {
    /// Support worker_ids as arithmetic progression:
    /// `vec![worker_id_base, worker_id_base + interval, ..., worker_id_base + interval * (number - 1)]`
    pub fn init(worker_id_base: u32, interval: u32, number: u32) {
        if number == 0 {
            panic!("Invalid number of instances");
        }
        unsafe {
            ID_BASE = worker_id_base;
            INTERVAL = interval;
        }
        let mut workers: Vec<u32> = Vec::new();
        if interval == 0 {
            workers.push(worker_id_base);
        } else {
            let mut id = worker_id_base;
            while unsafe { id <= ID_BASE + interval * (number - 1) } {
                workers.push(id);
                id += interval;
            }
        }
        let mut worker_vec: Vec<Arc<Mutex<DefaultIdGenerator>>> = Vec::new();
        worker_vec.resize(
            workers.len(),
            Arc::new(Mutex::new(DefaultIdGenerator::default())),
        );
        unsafe {
            ID_GEN_INSTANCE_VEC = Some(worker_vec);
        }
    }

    /// Set options
    fn id_gen_instance(worker_id: u32) -> Arc<Mutex<DefaultIdGenerator>> {
        let id: u32 = unsafe { (worker_id - ID_BASE) / INTERVAL };
        unsafe {
            ID_GEN_INSTANCE_VEC
                .as_ref()
                .unwrap()
                .get(id as usize)
                .unwrap()
                .clone()
        }
    }

    pub fn set_id_generator(options: IdGeneratorOptions) {
        let idgen_arc = IdVecHelper::id_gen_instance(options.worker_id);
        let mut idgen = idgen_arc.lock().unwrap();
        idgen.worker.set_options(options);
    }

    /// Set worker id of the one of the generators in Vector in instance
    pub fn set_worker_id(worker_id: u32) {
        let idgen_arc = IdVecHelper::id_gen_instance(worker_id);
        let mut idgen = idgen_arc.lock().unwrap();
        let options = IdGeneratorOptions::new(worker_id);
        idgen.worker.set_options(options);
    }

    /// Get a unique id of the generator with the given worker id
    pub fn next_id(worker_id: u32) -> i64 {
        let idgen_arc = IdVecHelper::id_gen_instance(worker_id);
        let mut idgen = idgen_arc.lock().unwrap();
        idgen.worker.next_id()
    }
}

// lazy_static! {
//     static ref ID_GEN_INSTANCE_MAP: Option<HashMap<u32, Arc<Mutex<DefaultIdGenerator>>>> =
//         None;
// }

/// multiple generators with different worker id in one instance, using `HashMap`
static mut ID_GEN_INSTANCE_MAP: Option<HashMap<u32, Arc<Mutex<DefaultIdGenerator>>>> = None;
pub struct IdMapHelper;
impl IdMapHelper {
    /// Support arbitrary worker_id at the cost of very little performance degradation
    pub fn init(workers: Vec<u32>) {
        let mut book: HashMap<u32, Arc<Mutex<DefaultIdGenerator>>> = HashMap::new();
        for worker_id in workers.iter() {
            book.insert(
                *worker_id,
                Arc::new(Mutex::new(DefaultIdGenerator::default())),
            );
        }
        unsafe {
            ID_GEN_INSTANCE_MAP = Some(book);
        }
    }

    fn id_gen_instance(worker_id: u32) -> Arc<Mutex<DefaultIdGenerator>> {
        unsafe {
            ID_GEN_INSTANCE_MAP
                .as_ref()
                .unwrap()
                .get(&worker_id)
                .unwrap()
                .clone()
        }
    }

    /// Set options
    pub fn set_id_generator(options: IdGeneratorOptions) {
        let idgen_arc = IdMapHelper::id_gen_instance(options.worker_id);
        let mut idgen = idgen_arc.lock().unwrap();
        idgen.worker.set_options(options);
    }

    /// Set worker id of the one of the generators in HashMap in instance
    pub fn set_worker_id(worker_id: u32) {
        let idgen_arc = IdMapHelper::id_gen_instance(worker_id);
        let mut idgen = idgen_arc.lock().unwrap();
        let options = IdGeneratorOptions::new(worker_id);
        idgen.worker.set_options(options);
    }

    /// Get a unique id of the generator with the given worker id
    pub fn next_id(worker_id: u32) -> i64 {
        let idgen_arc = IdMapHelper::id_gen_instance(worker_id);
        let mut idgen = idgen_arc.lock().unwrap();
        idgen.worker.next_id()
    }
}
