extern crate gstreamer as gst;


use std::env;
use std::process;
use std::time::{Duration, SystemTime};
use std::thread::sleep;

use gst::prelude::*;


fn main() {
    ////////////////////////////////////////
    // RTSP H264 video saver
    ////////////////////////////////////////

    let rtsp_url = env::args().skip(1).take(1).next().unwrap();

    gst::init().unwrap();
    // save RTSP video to local file
    let pipeline_str = format!(
      "rtspsrc location={} ! \
        rtph264depay ! h264parse ! mp4mux ! \
        filesink location=/tmp/test.mp4", rtsp_url);
    let mut context = gst::ParseContext::new();
    let pipeline =
        match gst::parse_launch_full(&pipeline_str,
                                     Some(&mut context),
                                     gst::ParseFlags::NONE) {
            Ok(pipeline) => pipeline,
            Err(err) => {
                if let Some(gst::ParseError::NoSuchElement) =
                        err.kind::<gst::ParseError>() {
                    println!("Missing element(s): {:?}",
                             context.get_missing_elements());
                } else {
                    println!("Failed to parse pipeline: {}", err);
                }

                process::exit(-1)
            }
        };
    let bus = pipeline.get_bus().unwrap();
    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    // let's record 5 seconds video
    let start_time = SystemTime::now();
    let duration = Duration::new(5, 0);

    while SystemTime::now().duration_since(start_time).unwrap() < duration {
        use gst::MessageView;
        if let Some(msg) = bus.timed_pop(gst::ClockTime(Some(100_000))) {
            match msg.view() {
                MessageView::Eos(..) => break,
                MessageView::Error(err) => {
                    println!(
                        "Error from {:?}: {} ({:?})",
                        msg.get_src().map(|s| s.get_path_string()),
                        err.get_error(),
                        err.get_debug()
                    );
                    break;
                }
                _ => (),
            };
        }
    }

    // sending eos
    pipeline.send_event(gst::Event::new_eos().build());

    // sleep 1 second, wait for the mp4 header
    sleep(Duration::new(1, 0));

    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}
