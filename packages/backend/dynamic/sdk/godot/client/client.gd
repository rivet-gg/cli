extends Node
class_name RivetClient
## Low-level API used to build HTTP requests to the backend.

const _ApiRequest := preload("request.gd")

var configuration: RivetConfiguration

func _init(configuration: RivetConfiguration):
	self.configuration = configuration

## Builds the headers for a request
func _build_headers() -> PackedStringArray:
	return [
		"Accept: application/json",
		"Content-Type: application/json",
	]

## Builds the complete URL to the backend
func _build_url(path: String) -> String:
	var path_segments := path.split("/", false)
	return self.configuration.endpoint + "/" + "/".join(path_segments)

## Creates a request
func build_request(request_name: String, method: HTTPClient.Method, path: String, body: Dictionary) -> _ApiRequest:
	if !self.is_inside_tree():
		push_error("RivetClient node not added to tree, cannot make http requests")

	var url := self._build_url(path)
	var body_json := JSON.stringify(body)

	return _ApiRequest.new(self, method, url, { 
		"headers": self._build_headers(), 
		"body": body_json,
		"request_name": request_name,
	})
