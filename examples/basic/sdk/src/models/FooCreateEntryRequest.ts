// This file is auto-generated by the Rivet (https://rivet.gg) build system.
//
// Do not edit this file directly.

/* tslint:disable */
/* eslint-disable */

/**
 * @export
 * @interface FooCreateEntryRequest
 */
export interface FooCreateEntryRequest {}

/**
 * Check if a given object implements the FooCreateEntryRequest interface.
 */
export function instanceOfFooCreateEntryRequest(
  _value: object,
): _value is FooCreateEntryRequest {
  return true;
}

export function FooCreateEntryRequestFromJSON(
  json: any,
): FooCreateEntryRequest {
  return FooCreateEntryRequestFromJSONTyped(json, false);
}

export function FooCreateEntryRequestFromJSONTyped(
  json: any,
  ignoreDiscriminator: boolean,
): FooCreateEntryRequest {
  if (json == null) {
    return json;
  }
  return {};
}

export function FooCreateEntryRequestToJSON(
  value?: FooCreateEntryRequest | null,
): any {
  if (value == null) {
    return value;
  }
  return {};
}