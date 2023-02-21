use futures::executor::block_on;
use jobst_common::job::{Job, JobType, State, ExecStatus, JobPlan};
use jobst_common::adapter;
use jobst_common::adapter::AdapterError;
use jobst_common::config;

pub struct Listener {
    engine_type: config::EngineType,
    adapter: Box<dyn adapter::JobAdapterTrait>,

}

impl Listener {
    pub fn new() -> Listener {
        let adapter_type: adapter::AdapterType = config::CONFIG.engine_type.clone().into();
        let adapter_obj = adapter::new_job_adapter(adapter_type);

        Listener {
            engine_type: config::CONFIG.engine_type.clone(),
            adapter: adapter_obj,
        }
    }
}

impl Listener {
    pub fn init_job_plan(&self) {
        let mut c = true;
        let mut count = 0;
        let async_block = async {
            let r = self.adapter.get_plan_list().await;
            r
        };
        let r  = block_on(async_block);
        match r {
            Ok(plans)=>{
                for plan in plans {

                }
            },
            Err(e)=>{  panic!("{}", e.to_string())}

        }

    }
}

