#[macro_use] extern crate rocket;
use libcamera::{camera_manager::CameraManager, logging::LoggingLevel, stream::StreamRole};

#[get("/")]
fn index() -> &'static str {
    "Index page running on PiZeroW2 at cam4.local."
}

#[get("/camlist")]
fn camlist() -> String {

    println!("running camlist");

    let mut pageout = String::new();
    pageout.push_str("List of Cameras: none");

    // @TODO linker needs sysroot with libcamera installed?
    // let mgr = CameraManager::new().unwrap();

    // mgr.log_set_level("Camera", LoggingLevel::Error);

    // let cameras = mgr.cameras();

    // for i in 0..cameras.len() {
    //     let cam = cameras.get(i).unwrap();
    //     pageout.push_str(&format!("Camera {}", i));
    //     pageout.push_str(&format!("ID: {}", cam.id()));

    //     pageout.push_str(&format!("Properties: {:#?}", cam.properties()));

    //     let config = cam.generate_configuration(&[StreamRole::ViewFinder]).unwrap();
    //     let view_finder_cfg = config.get(0).unwrap();
    //     pageout.push_str(&format!("Available formats: {:#?}", view_finder_cfg.formats()));
    // }


    pageout

}

#[get("/campic")]
fn campic() -> String {

    println!("running campic");
    let mut pageout = String::new();
    pageout.push_str("List of Pictures: none");

    pageout
}

#[launch]
fn rocket() -> _ {
    println!("Starting Rocket server on PiZeroW2 at cam4.local.");
    rocket::build()
        .mount("/", routes![index,camlist,campic])
}

