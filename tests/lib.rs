// Copyright 2015 MaidSafe.net limited.
//
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate routing;
extern crate sodiumoxide;

use std::error::Error;

fn start_nodes(number_of_nodes: usize) -> Vec<::std::process::Child> {
    env_logger::init().unwrap_or_else(|e| info!("Error initialising logger: {:?}", e));
    let mut processes = Vec::new();
    let executable_path = match std::env::current_exe() {
        Ok(mut exe_path) => {
            exe_path.pop();
            std::path::Path::new("./target").join(exe_path.iter().last().unwrap()).join("node")
        }
        Err(e) => panic!("Failed to get current integration test path: {}", e),
    };

    debug!("Expecting node executable at path {}", executable_path.to_path_buf().display());

    for _ in 0..number_of_nodes {
        processes.push(
            match ::std::process::Command::new(
                executable_path.to_path_buf()).stderr(::std::process::Stdio::piped()).spawn() {
                    Err(e) => panic!("Failed to spawn process: {}", e.description()),
                    Ok(process) => {
                        debug!("Starting Node {:05}", process.id());
                    	process
                    }
            });
        ::std::thread::sleep_ms(1000);
    }
    ::std::thread::sleep_ms(1000);
    processes
}

fn stop_nodes(processes: &mut Vec<::std::process::Child>) {
	while let Some(mut process) = processes.pop() {
		debug!("Stopping Node {:05}", process.id());
        let _ = process.kill();
    }
}

fn calculate_key_name(key: &::std::string::String) -> ::routing::NameType {
    ::routing::NameType::new(::sodiumoxide::crypto::hash::sha512::hash(key.as_bytes()).0)
}

#[cfg(test)]
mod test {

	#[test]
	fn start_stop_nodes() {
	    let mut nodes = super::start_nodes(8usize);
		super::stop_nodes(&mut nodes);
	}

	#[test]
	fn start_stop_client() {
        let mut client = ::routing::test_utils::client::Client::new();
        client.run();
        client.stop();
	}

    #[test]
    fn client_put_get() {
        let mut nodes = super::start_nodes(8usize);
        let client = ::routing::test_utils::client::Client::new();

        let key = ::std::string::String::from("key");
        let value = ::std::string::String::from("value");

        let name = super::calculate_key_name(&key.clone());
        let data = ::routing::utils::encode(&(key.clone(), value)).unwrap();
        let data = ::routing::data::Data::PlainData(
                ::routing::plain_data::PlainData::new(name.clone(), data));

        client.put(data.clone());

        // let recovered_data = match client.get(::routing::data::DataRequest::PlainData(name)) {
        //     Some(data) => data,
        //     None => panic!("Failed to recover stored data: {}.", name),
        // };

        // assert_eq!(recovered_data, data);

        super::stop_nodes(&mut nodes);
    }
}
