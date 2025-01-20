extern crate js_sys;
extern crate osm_xml as osm;
extern crate wasm_bindgen;
extern crate web_sys;
use std::f64;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    log(&format!("Hello, {}!", name));

    "Hello, World!".to_string()
}

fn map_points(value: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    ((value - start1) / (stop1 - start1) * (stop2 - start2) + start2).floor()
}

fn process_points(
    node: &osm::Node,
    bounds: &osm::Bounds,
    width: f64,
    height: f64,
) -> js_sys::Array {
    let y = map_points(node.lat, bounds.minlat, bounds.maxlat, 0.0, width);
    let x = map_points(node.lon, bounds.minlon, bounds.maxlon, 0.0, height);
    let point = js_sys::Array::new();
    point.push(&JsValue::from_f64(x));
    point.push(&JsValue::from_f64(y));
    point
}

fn parse_coords(
    doc: &osm::OSM,
    way: &osm::Way,
    bounds: &osm::Bounds,
    width: f64,
    height: f64,
) -> js_sys::Array {
    let coords = js_sys::Array::new();
    for node in way.nodes.iter() {
        let n = &doc.resolve_reference(&node);
        match n {
            osm::Reference::Node(node) => {
                let point = process_points(node, &bounds, width, height);
                coords.push(&point);
            }
            _ => {}
        }
    }
    coords
}

#[wasm_bindgen]
pub fn parse_nodes(text: String, width: f64, height: f64) -> js_sys::Array {
    let doc = osm::OSM::parse(text.as_bytes()).unwrap();
    let bounds = doc.bounds.unwrap();
    let arr = js_sys::Array::new();
    for (_id, node) in doc.nodes.iter() {
        arr.push(&process_points(node, &bounds, width, height));
    }
    arr
}

#[wasm_bindgen]
pub fn parse_ways(text: String, width: f64, height: f64) -> js_sys::Array {
    let doc = osm::OSM::parse(text.as_bytes()).unwrap();
    let bounds = doc.bounds.unwrap();
    let arr = js_sys::Array::new();
    for (_id, way) in doc.ways.iter() {
        arr.push(&parse_coords(&doc, way, &bounds, width, height));
    }
    arr
}

#[wasm_bindgen]
pub fn parse_relations(text: String, width: f64, height: f64) -> js_sys::Array {
    let doc = osm::OSM::parse(text.as_bytes()).unwrap();
    let bounds = doc.bounds.unwrap();
    let arr = js_sys::Array::new();
    for (_id, relation) in doc.relations.iter() {
        for member in relation.members.iter() {
            match member {
                osm::Member::Way(way, t) => {
                    let w = &doc.resolve_reference(&way);
                    match w {
                        osm::Reference::Way(way) => {
                            // Only processing coordinates for the "outer" way.
                            if t == "outer" {
                                arr.push(&&parse_coords(&doc, way, &bounds, width, height));
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    arr
}

///////////////////////////////////////////////
///////////////////////////////////////////////
///////////////////////////////////////////////


// async fn make_request(bbox: &str, query: &str) -> Result<JsValue, JsValue> {
//     let opts = RequestInit::new();
//     opts.set_method("GET");
//     opts.set_mode(RequestMode::Cors);
//     let url = format!("https://overpass-api.de/api/interpreter?data=[timeout:3600][maxsize:1073741824][bbox:{}];{};out;", bbox, query);
//
//     let request = Request::new_with_str_and_init(&url, &opts)?;
//     let window = web_sys::window().unwrap();
//     let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
//     assert!(resp_value.is_instance_of::<Response>());
//     let resp: Response = resp_value.dyn_into().unwrap();
//     let value = JsValue::from(JsFuture::from(resp.text()?).await?);
//     Ok(value)
// }

// fn listen_for_position(window: &Window) {
//     let geolocation = window
//         .navigator()
//         .geolocation()
//         .expect("Unable to get geolocation.");
//
//     let success_callback = Closure::wrap(Box::new(move |position: Position| {
//         let coords = position.coords();
//         let lat = coords.latitude();
//         let lon = coords.longitude();
//         unsafe {
//             LAT = lat;
//             LON = lon;
//         }
//         log("Got position data!");
//     }) as Box<dyn FnMut(Position)>);
//
//     let error_callback = Closure::wrap(Box::new(move |err: PositionError| {
//         log("Error getting position data!");
//     }) as Box<dyn FnMut(PositionError)>);
//
//     geolocation
//         .get_current_position_with_error_callback(
//             success_callback.as_ref().unchecked_ref(),
//             Some(error_callback.as_ref().unchecked_ref()),
//         )
//         .unwrap();
//
//     success_callback.forget();
//     error_callback.forget();
// }

//     function drawMap() {
//       if (buildings_data) {
//         for (let building of buildings_data) {
//           ctx.beginPath();
//           ctx.fillStyle = "#cccccc";
//           for (let point of building) {
//             ctx.lineTo(point[0], point[1]);
//           }
//           ctx.fill();
//           ctx.closePath();
//         }
//       }
//       if (footways_data) {
//         for (let footway of footways_data) {
//           ctx.beginPath();
//           ctx.strokeStyle = "#ccaacc";
//           for (let point of footway) {
//             ctx.lineTo(point[0], point[1]);
//           }
//           ctx.stroke();
//           ctx.closePath();
//         }
//       }
//       if (highways_data) {
//         for (let highway of highways_data) {
//           ctx.beginPath();
//           ctx.strokeStyle = "#aaaaaa";
//           for (let point of highway) {
//             ctx.lineTo(point[0], point[1]);
//           }
//           ctx.stroke();
//           ctx.closePath();
//         }
//       }
//       if (sidewalks_data) {
//         for (let sidewalk of sidewalks_data) {
//           ctx.beginPath();
//           ctx.strokeStyle = "#ffaaaa";
//           for (let point of sidewalk) {
//             ctx.lineTo(point[0], point[1]);
//           }
//           ctx.stroke();
//           ctx.closePath();
//         }
//       }
//       if (cycleways_data) {
//         for (let cycleway of cycleways_data) {
//           ctx.beginPath();
//           ctx.strokeStyle = "#ffaaaa";
//           for (let point of cycleway) {
//             ctx.lineTo(point[0], point[1]);
//           }
//           ctx.stroke();
//           ctx.closePath();
//         }
//       }
//       if (bus_routes_data) {
//         for (let bus_route of bus_routes_data) {
//           ctx.beginPath();
//           ctx.strokeStyle = "#aaaaff";
//           for (let point of bus_route) {
//             ctx.lineTo(point[0], point[1]);
//           }
//           ctx.stroke();
//           ctx.closePath();
//         }
//       }
//       if (bus_stops_data) {
//         for (let bus_stop of bus_stops_data) {
//           ctx.fillStyle = "#aaaaff";
//           ctx.strokeStyle = "#aaaaaa";
//           ctx.beginPath();
//           ctx.arc(bus_stop[0], bus_stop[1], 3, 0, 2 * Math.PI);
//           ctx.fill();
//           ctx.stroke();
//           ctx.closePath();
//         }
//       }
//     }
//

// enum LayerType {
//     Path,
//     Area,
//     Point,
// }
//
// struct Color {
//     fill: JsValue,
//     stroke: JsValue,
// }
//
// struct Layer {
//     query: String,
//     layer_type: LayerType,
//     data: js_sys::Array,
//     color: Color,
// }
//
// trait Map {
//     fn init(&self);
//     fn draw(&self, layers: &Vec<Layer>);
// }
//
// impl Map for web_sys::HtmlCanvasElement {
//     fn init(&self) {
//         let ctx = self
//             .get_context("2d")
//             .unwrap()
//             .unwrap()
//             .dyn_into::<CanvasRenderingContext2d>()
//             .unwrap();
//         ctx.scale(1.0, -1.0).unwrap();
//
//         ctx.begin_path();
//         ctx.rect(0.0, 0.0, self.width() as f64, self.height() as f64);
//         ctx.fill();
//         ctx.close_path();
//     }
//
//     fn draw(&self, layers: &Vec<Layer>) {
//         let ctx = self
//             .get_context("2d")
//             .unwrap()
//             .unwrap()
//             .dyn_into::<CanvasRenderingContext2d>()
//             .unwrap();
//
//         for layer in layers.iter() {
//             ctx.begin_path();
//             ctx.set_fill_style(&layer.color.fill);
//             ctx.set_stroke_style(&layer.color.stroke);
//             match layer.layer_type {
//                 LayerType::Path => {
//                     for i in 0..layer.data.length() {
//                         let coords = layer.data.get(i).dyn_ref::<js_sys::Array>().unwrap().clone();
//                         let x = coords.get(0).as_f64().unwrap();
//                         let y = coords.get(1).as_f64().unwrap();
//                         if i == 0 {
//                             ctx.move_to(x, y);
//                         } else {
//                             ctx.line_to(x, y);
//                         }
//                     }
//                 }
//                 LayerType::Area => {
//                   for i in 0..layer.data.length() {
//                     let coords = layer.data.get(i).dyn_ref::<js_sys::Array>().unwrap().clone();
//                     ctx.move_to(coords.get(0).as_f64().unwrap(), coords.get(1).as_f64().unwrap());
//                     for j in 1..coords.length() {
//                       let x = coords.get(j).as_f64().unwrap();
//                       let y = coords.get(j + 1).as_f64().unwrap();
//                       ctx.line_to(x, y);
//                     }
//                   }
//                 }
//                 LayerType::Point => {
//                     for i in 0..layer.data.length() {
//                         let coords = layer.data.get(i).dyn_ref::<js_sys::Array>().unwrap().clone();
//                         let x = coords.get(0).as_f64().unwrap();
//                         let y = coords.get(1).as_f64().unwrap();
//                         ctx.begin_path();
//                         ctx.arc(x, y, 3.0, 0.0, 2.0 * f64::consts::PI).unwrap();
//                     }
//                 }
//             }
//             ctx.fill();
//             ctx.stroke();
//             ctx.close_path();
//         }
//     }
// }

// #[wasm_bindgen(start)]
// async fn start() -> Result<(), JsValue> {
//     let window = web_sys::window().expect("should have a window in this context");
//     let document = window.document().expect("window should have a document");
//
//     let mut width: f64 = 500.0;
//     let mut height: f64 = 500.0;
//
//
//     let canvas = document.create_element("canvas").unwrap();
//     let map = canvas.dyn_ref::<HtmlCanvasElement>().unwrap();
//     map.set_id("map");
//     map.set_class_name("map");
//     map.set_width(width as u32);
//     map.set_height(height as u32);
//     document.body().unwrap().append_child(&map).unwrap();
//
//
//     let lat = 52.408027;
//     let lon = -1.5124486;
//     let range = 0.015;
//     let n = lat + range;
//     let s = lat - range;
//     let e = lon + range;
//     let w = lon - range;
//     let bbox = format!("{},{},{},{}", s, w, n, e);
//     let ratio = (n - s) / (e - w);
//     width = window.inner_width().unwrap().as_f64().unwrap();
//     height = width * ratio;
//
//     map.set_width(width as u32);
//     map.set_height(height as u32);
//
//     map.init();
//
//     // let buildings = "way[building];foreach{(._;>;);out;}";
//     // let footways = "way[highway=footway];foreach{(._;>;);out;}";
//     // let highways = "way[highway];foreach{(._;>;);out;}";
//     // let sidewalks = "way[sidewalk];foreach{(._;>;);out;}";
//     // let cycleways = "way[cycleway];foreach{(._;>;);out;}";
//     // let bus_routes_query = "relation[type=route][route=bus];foreach{(._;>;);out;}";
//     // let bus_stops = "node[highway=bus_stop];foreach{(._;>;);out;}";
//
//     let layers = vec![
//       Layer {
//         query: "way[landuse];foreach{(._;>;);out;}".to_string(),
//         layer_type: LayerType::Area,
//         color: Color {
//           fill: JsValue::from_str("#333333"),
//           stroke: JsValue::from_str("#333333"),
//         },
//         data: js_sys::Array::new(),
//       },
//       Layer {
//         query: "way[water];foreach{(._;>;);out;}".to_string(),
//         layer_type: LayerType::Area,
//         color: Color {
//           fill: JsValue::from_str("#0099ff"),
//           stroke: JsValue::from_str("#0099ff"),
//         },
//         data: js_sys::Array::new(),
//       },
//     ];
//
//     let mut i = 0;
//     while i < layers.len() {
//         let layer = &layers[i];
//         let request = make_request(&bbox, &layer.query).await?;
//         let text = request.as_string().unwrap();
//         match layer.layer_type {
//             LayerType::Area => {
//                 let data = parse_ways(text, width, height);
//                 for i in 0..data.length() {
//                     layer.data.push(&data.get(i));
//                 }
//             }
//             _ => {}
//         }
//         i += 1;
//     }
//
//     map.draw(&layers);
//     Ok(())
// }
