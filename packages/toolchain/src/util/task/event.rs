use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum TaskEvent {
	#[serde(rename = "log")]
	Log(String),
	#[serde(rename = "result")]
	Result {
		result: Box<serde_json::value::RawValue>,
	},
	#[serde(rename = "port_update")]
	PortUpdate { backend_port: u16, editor_port: u16 },
	#[serde(rename = "backend_config_update")]
	BackendConfigUpdate(backend_config_update::Event),
}

pub mod backend_config_update {
	use serde::Serialize;

	#[derive(Serialize, Debug)]
	pub struct Event {
		pub sdks: Vec<Sdk>,
		pub modules: Vec<Module>,
	}

	#[derive(Serialize, Debug)]
	pub struct Sdk {
		pub target: String,
		pub output: String,
	}

	#[derive(Serialize, Debug)]
	pub struct Module {
		pub slug: String,
		pub name: String,
		pub config_url: String,
		pub docs_url: String,
	}
}
