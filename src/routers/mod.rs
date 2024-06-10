mod controllers;

use crate::controllers::{};
use axum::{routing:{get,post},Router};



pub fn Routers()->Router{
  const API_ROUTE:&str = String::from("/api/v1/");
  const CLIENT_ROUTE:&str ="/"
  Router::new()
  ///api routes
   .route(API_ROUTE.push("/dec"),post())
   .route(API_ROUTE.push("/enc"),post())
   .route(API_ROUTE.push("/stefrwrd"),post())
   .route(API_ROUTE.push("/steinv"),post())
   
   ///client routes 
   .route(CLIENT_ROUTE.push("steganograpghy"),get())
   .route(CLIENT_ROUTE.push("encryption"),get())
   .route(CLIENT_ROUTE.push("exif"),get())
   .route(CLIENT_ROUTE.push("home"),get())
   .route(CLIENT_ROUTE,get())
   
}