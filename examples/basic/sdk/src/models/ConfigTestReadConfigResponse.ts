// This file is auto-generated by the Rivet (https://rivet.gg) build system.
//
// Do not edit this file directly.

/* tslint:disable */
/* eslint-disable */

/**
 * @export
 * @interface ConfigTestReadConfigResponse
 */
export interface ConfigTestReadConfigResponse {}

/**
 * Check if a given object implements the ConfigTestReadConfigResponse interface.
 */
export function instanceOfConfigTestReadConfigResponse(
  _value: object,
): _value is ConfigTestReadConfigResponse {
  return true;
}

export function ConfigTestReadConfigResponseFromJSON(
  json: any,
): ConfigTestReadConfigResponse {
  return ConfigTestReadConfigResponseFromJSONTyped(json, false);
}

export function ConfigTestReadConfigResponseFromJSONTyped(
  json: any,
  ignoreDiscriminator: boolean,
): ConfigTestReadConfigResponse {
  if (json == null) {
    return json;
  }
  return {};
}

export function ConfigTestReadConfigResponseToJSON(
  value?: ConfigTestReadConfigResponse | null,
): any {
  if (value == null) {
    return value;
  }
  return {};
}