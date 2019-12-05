use std::iter;
use std::thread;

use serde_derive::{Deserialize, Serialize};
use yew::{worker::*, services::ConsoleService};

use day03::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    GetPart1,
    GetPart2,
    GetWireInfos,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Part1(u32),
    Part2(u32),
    WireInfo1(Point),
    WireInfo2(Point),
}

pub struct Worker {
    console: ConsoleService,
    link: AgentLink<Worker>,
}

impl Agent for Worker {
    type Reach = Public;
    type Message = ();
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        let mut console = ConsoleService::new();
        console.log("create");
        
        Self {
            console,
            link,
        }
    }

    fn update(&mut self, _: Self::Message) {
        self.console.log("update");
    }

    fn handle(&mut self, request: Self::Input, who: HandlerId) {
        self.console.log(&format!("handle {:?} thread {:?}", request, thread::current().id()));
        
        match request {
            Request::GetPart1 => self.link.response(who, Response::Part1(part_1())),
            Request::GetPart2 => self.link.response(who, Response::Part2(part_2())),
            Request::GetWireInfos => {
                let mut data = INPUT
                    .lines()
                    .map(|l| Wire::new(l.trim().split(SEPARATOR).collect::<Vec<&str>>()))
                    .collect::<Vec<Wire>>();

                loop {
                    match (data[0].next(), data[1].next()) {
                        (Some(p1), Some(p2)) => {
                            self.link.response(who, Response::WireInfo1(p1));
                            self.link.response(who, Response::WireInfo2(p2));
                        }
                        (Some(p1), None) => {
                            self.link.response(who, Response::WireInfo1(p1));
                        }
                        (None, Some(p2)) => {
                            self.link.response(who, Response::WireInfo2(p2));
                        }
                        (None, None) => break,
                    }
                }
            }
        }
    }

    fn name_of_resource() -> &'static str {
        "bin/worker.js"
    }
}

struct Wire<'a> {
    current: Point,
    path: Box<dyn Iterator<Item=char> + 'a>,
}

impl<'a> Wire<'a> {
    fn new(path: Vec<&'a str>) -> Self {
        Self {
            current: Point::new(0, 0),
            path: Box::new(path
                           .into_iter()
                           .flat_map(|s| {
                               let mut i = s.chars();
                               let c = i.next().unwrap();
                               let n = i.collect::<String>().parse().unwrap();
                               iter::repeat(c).take(n)
                           })),
                           
        }
    }
}

impl<'a> Iterator for Wire<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        match self.path.next() {
            Some('D') => {
                self.current += Point::new(0, -1);
                Some(self.current)
            }
            Some('U') => {
                self.current += Point::new(0, 1);
                Some(self.current)
            }
            Some('L') => {
                self.current += Point::new(-1, 0);
                Some(self.current)
            }
            Some('R') => {
                self.current += Point::new(1, 0);
                Some(self.current)
            }
            _ => None,
        }
    }
}
