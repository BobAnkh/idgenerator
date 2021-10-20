use crate::idgen::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

pub struct IdHelper;

static mut ID_GEN_INSTANCE: Option<Arc<Mutex<DefaultIdGenerator>>> = None;

// lazy_static! {
//     static ref ID_GEN_INSTANCE_MAP: Option<HashMap<u32, Arc<Mutex<DefaultIdGenerator>>>> =
//         None;
// }

static mut ID_GEN_INSTANCE_MAP: Option<HashMap<u32, Arc<Mutex<DefaultIdGenerator>>>> = None;

// lazy_static! {
//     static ref ID_GEN_INSTANCE_VEC: Option<Vec<Arc<Mutex<DefaultIdGenerator>>>> =
//         None;
// }

static mut ID_GEN_INSTANCE_VEC: Option<Vec<Arc<Mutex<DefaultIdGenerator>>>> = None;

static mut ID_BASE: u32 = 1;
static mut INTERVAL: u32 = 1;

impl IdHelper {
    /// One instance only
    pub fn init() {
        unsafe {
            ID_GEN_INSTANCE = Some(Arc::new(Mutex::new(DefaultIdGenerator::default())));
        }
    }

    fn id_gen_instance() -> Arc<Mutex<DefaultIdGenerator>> {
        unsafe { ID_GEN_INSTANCE.as_ref().unwrap().clone() }
    }

    pub fn set_id_generator(options: IdGeneratorOptions) {
        let idgen_arc = IdHelper::id_gen_instance();
        let mut idgen = idgen_arc.lock().unwrap();
        idgen.worker.set_options(options);
    }

    pub fn set_worker_id(worker_id: u32) {
        let idgen_arc = IdHelper::id_gen_instance();
        let mut idgen = idgen_arc.lock().unwrap();
        let options = IdGeneratorOptions::new(worker_id);
        idgen.worker.set_options(options);
    }

    pub fn next_id() -> i64 {
        let idgen_arc = IdHelper::id_gen_instance();
        let mut idgen = idgen_arc.lock().unwrap();
        idgen.worker.next_id()
    }

    /// Support arbitrary worker_id at the cost of performance degradation
    pub fn init_map(workers: Vec<u32>) {
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

    fn id_gen_instance_map(worker_id: u32) -> Arc<Mutex<DefaultIdGenerator>> {
        unsafe {
            ID_GEN_INSTANCE_MAP
                .as_ref()
                .unwrap()
                .get(&worker_id)
                .unwrap()
                .clone()
        }
    }

    pub fn set_id_generator_map(options: IdGeneratorOptions) {
        let idgen_arc = IdHelper::id_gen_instance_map(options.worker_id);
        let mut idgen = idgen_arc.lock().unwrap();
        idgen.worker.set_options(options);
    }

    pub fn set_worker_id_map(worker_id: u32) {
        let idgen_arc = IdHelper::id_gen_instance_map(worker_id);
        let mut idgen = idgen_arc.lock().unwrap();
        let options = IdGeneratorOptions::new(worker_id);
        idgen.worker.set_options(options);
    }

    pub fn next_id_map(worker_id: u32) -> i64 {
        let idgen_arc = IdHelper::id_gen_instance_map(worker_id);
        let mut idgen = idgen_arc.lock().unwrap();
        idgen.worker.next_id()
    }

    /// Support worker_ids as arithmetic progression: 
    /// vec![worker_id_base, worker_id_base + interval, ..., worker_id_base + interval * (number - 1)]
    pub fn init_vec(worker_id_base: u32, interval: u32, number: u32) {
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

    fn id_gen_instance_vec(worker_id: u32) -> Arc<Mutex<DefaultIdGenerator>> {
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

    pub fn set_id_generator_vec(options: IdGeneratorOptions) {
        let idgen_arc = IdHelper::id_gen_instance_vec(options.worker_id);
        let mut idgen = idgen_arc.lock().unwrap();
        idgen.worker.set_options(options);
    }

    pub fn set_worker_id_vec(worker_id: u32) {
        let idgen_arc = IdHelper::id_gen_instance_vec(worker_id);
        let mut idgen = idgen_arc.lock().unwrap();
        let options = IdGeneratorOptions::new(worker_id);
        idgen.worker.set_options(options);
    }

    pub fn next_id_vec(worker_id: u32) -> i64 {
        let idgen_arc = IdHelper::id_gen_instance_vec(worker_id);
        let mut idgen = idgen_arc.lock().unwrap();
        idgen.worker.next_id()
    }
}
