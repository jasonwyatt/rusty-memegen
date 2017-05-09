extern crate iron;
extern crate time;

use time::precise_time_ns;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::prelude::*;
use iron::request::Request;

pub struct ResponseTime;

impl typemap::Key for ResponseTime { 
    type Value = u64; 
}

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        info!("{} took: {} ms", req.url, (delta as f64) / 1000000.0);
        Ok(res)
    }
    fn catch(&self, request: &mut Request, err: IronError) -> IronResult<Response> {
        error!("Error happened: {}", err);
        error!("Request was: {:?}", request);
        Err(err)
    }
}