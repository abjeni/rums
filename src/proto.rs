

pub mod proto {

    extern crate prost_build;

    pub struct Generator {

    }

    // stolen from tonic
    fn naive_snake_case(name: &str) -> String {
        let mut s = String::new();
        let mut it = name.chars().peekable();

        while let Some(x) = it.next() {
            s.push(x.to_ascii_lowercase());
            if let Some(y) = it.peek() {
                if y.is_uppercase() {
                    s.push('_');
                }
            }
        }

        s
    }

    impl prost_build::ServiceGenerator for Generator {
        fn generate(&mut self, service: prost_build::Service, buf: &mut String) {

            buf.push_str("\n");
            buf.push_str(format!("// service: {}\n", service.proto_name).as_str());
            buf.push_str(format!("pub mod {} {{\n", naive_snake_case(service.name.as_str())).as_str());
            {

                // Client

                buf.push_str("\n");
                buf.push_str("\tuse rums::Configuration;\n");
                buf.push_str("\n");
                buf.push_str("\tuse futures::stream::Stream;\n");
                buf.push_str("\n");
                buf.push_str("\tuse std::error::Error;\n");
                buf.push_str("\n");
                buf.push_str("\tuse futures::stream::StreamExt;\n");
                buf.push_str("\n");
                buf.push_str("\tuse prost::Message;\n");
                buf.push_str("\n");
                buf.push_str("\tuse rums::ServerHandler;\n");
                buf.push_str("\n");
                buf.push_str("\tuse rums::get_route;\n");
                buf.push_str("\tuse rums::add_route;\n");
                buf.push_str("\tuse rums::RouteHandler;\n");
                buf.push_str("\tuse rums::Response;\n");
                buf.push_str("\n");
                buf.push_str("\tuse std::marker::Copy;\n");
                buf.push_str("\n");

                buf.push_str(format!("\tpub trait {}Client<NIDT> {{\n", service.name).as_str());
                {

                    for method in service.methods.iter() {
                        buf.push_str(format!("\t\tfn {}<'a>(&'a self, msg: &super::{}) -> impl Stream<Item = Response<'a, super::{}, NIDT>> where NIDT: 'a;\n", method.name, method.input_type, method.output_type).as_str());
                    }

                }
                buf.push_str("\t}\n");
                buf.push_str("\n");

                buf.push_str(format!("\timpl<NIDT: Copy> {}Client<NIDT> for Configuration<NIDT> {{\n", service.name).as_str());
                {

                    for method in service.methods.iter() {
                        buf.push_str("\n");
                        buf.push_str(format!("\t\t// method: {}\n", method.proto_name).as_str());
                        buf.push_str(format!("\t\tfn {}<'a>(&'a self, msg: &super::{}) -> impl Stream<Item = Response<'a, super::{}, NIDT>> where NIDT: 'a {{\n", method.name, method.input_type, method.output_type).as_str());
                        {
                            buf.push_str("\t\t\tlet mut buf = vec![];\n");
                            buf.push_str("\t\t\tmsg.encode(&mut buf).unwrap();\n");
                            buf.push_str(format!("\t\t\tlet buf = add_route(add_route(add_route(buf, \"{}\"), \"{}\"), \"{}\");\n", method.proto_name, service.proto_name, service.package).as_str());
                            buf.push_str("\n");
                            buf.push_str("\t\t\tlet responses = self.send(buf);\n");
                            buf.push_str("\n");
                            buf.push_str("\t\t\tresponses.map(|res| {\n");
                            buf.push_str("\t\t\t\tResponse {\n");
                            buf.push_str("\t\t\t\t\tnode: res.node,\n");
                            buf.push_str("\t\t\t\t\tresponse: res.response.map(|buf| {\n");
                            buf.push_str(format!("\t\t\t\t\t\tsuper::{}::decode(&buf as &[u8]).unwrap()\n", method.output_type).as_str());
                            buf.push_str("\t\t\t\t\t})\n");
                            buf.push_str("\t\t\t\t}\n");
                            buf.push_str("\t\t})\n");
                        }
                        buf.push_str("\t\t}\n");
                    }

                    

                }
                buf.push_str("\n");
                buf.push_str("\t}\n");
                buf.push_str("\n");

                // Server

                buf.push_str(format!("\tpub trait {}Server {{\n", service.name).as_str());
                {

                    for method in service.methods.iter() {
                        buf.push_str(format!("\t\tfn {}(&mut self, msg: super::{}) -> Result<super::{}, Box<dyn Error + Send>>;\n", method.name, method.input_type, method.output_type).as_str());
                    }

                }
                buf.push_str("\t}\n");
                buf.push_str("\n");

                buf.push_str(format!("\tpub struct {}Handler {{\n", service.name).as_str());
                {
                    buf.push_str(format!("\t    handlers: Box<dyn {}Server + Send>\n", service.name).as_str());
                }
                buf.push_str("\t}\n");
                buf.push_str("\n");

                buf.push_str(format!("\timpl {}Handler {{\n", service.name).as_str());
                {
                    buf.push_str(format!("\t\tpub fn new(handlers: Box<dyn {}Server + Send>) -> Self {{\n", service.name).as_str());
                    {
                        buf.push_str("\t\t\tSelf {\n");
                        {
                            buf.push_str("\t\t\t\thandlers: handlers\n");
                        }
                        buf.push_str("\t\t\t}\n");
                    }
                    buf.push_str("\t\t}\n");
                }
                buf.push_str("\t}\n");
                buf.push_str("\n");

                buf.push_str(format!("\timpl ServerHandler for {}Handler {{\n", service.name).as_str());
                {
                    buf.push_str("\t\tfn handle(&'_ mut self, request: &[u8]) -> Result<Vec<u8>, Box<dyn Error + Send>> {\n");
                    {
                        buf.push_str("\t\t\tlet (request, route) = get_route(request);\n");
                        buf.push_str("\n");
                        buf.push_str("\t\t\tmatch route {\n");
                        for method in service.methods.iter() {
                            buf.push_str(format!("\t\t\t\t\"{}\" => {{\n", method.proto_name).as_str());
                            {
                                buf.push_str(format!("\t\t\t\t\tlet msg = super::{}::decode(request).unwrap();\n", method.input_type).as_str());
                                buf.push_str(format!("\t\t\t\t\tlet rsp = self.handlers.{}(msg);\n", method.name).as_str());
                                
                                buf.push_str("\t\t\t\t\trsp.map(|msg| {\n");
                                {
                                    buf.push_str("\t\t\t\t\t\tlet mut buf = vec![];\n");
                                    buf.push_str("\t\t\t\t\t\tmsg.encode(&mut buf).unwrap();\n");
                                    buf.push_str("\t\t\t\t\t\tbuf\n");
                                }
                                buf.push_str("\t\t\t\t\t})\n");
                            }

                            buf.push_str("\t\t\t\t},\n");
                        }
                        buf.push_str("\t\t\t\t&_ => todo!()\n");
                        buf.push_str("\t\t\t}\n");
                    }
                    buf.push_str("\t\t}\n");
                }
                buf.push_str("\t}\n");
                buf.push_str("\n");

                buf.push_str(format!("\tpub trait Register{}Handler {{\n", service.name).as_str());
                {
                    buf.push_str(format!("\t\tfn register_{}_handler(&mut self, handler: Box<dyn ServerHandler + Send>);\n", naive_snake_case(service.name.as_str())).as_str());
                }
                buf.push_str("\t}\n");
                buf.push_str("\n");

                buf.push_str(format!("\timpl Register{}Handler for RouteHandler {{\n", service.name).as_str());
                {
                    buf.push_str(format!("\t\tfn register_{}_handler(&mut self, handler: Box<dyn ServerHandler + Send>) {{\n", naive_snake_case(service.name.as_str())).as_str());
                    buf.push_str(format!("\t\t\tself.sub_route(\"{}\").add_route(\"{}\", handler);\n", service.package, service.proto_name).as_str());
                    buf.push_str("\t\t}\n");
                }
                buf.push_str("\t}\n");
                buf.push_str("\n");

            }
            buf.push_str("\n");
            buf.push_str("}\n");

        }
    }
}