// This file is auto-generated by the Rivet (https://rivet.gg) build system.
//
// Do not edit this file directly.

/* tslint:disable */
/* eslint-disable */

/**
 * @export
 * @interface UsersCreateTokenResponse
 */
export interface UsersCreateTokenResponse {}

/**
 * Check if a given object implements the UsersCreateTokenResponse interface.
 */
export function instanceOfUsersCreateTokenResponse(
  _value: object,
): _value is UsersCreateTokenResponse {
  return true;
}

export function UsersCreateTokenResponseFromJSON(
  json: any,
): UsersCreateTokenResponse {
  return UsersCreateTokenResponseFromJSONTyped(json, false);
}

export function UsersCreateTokenResponseFromJSONTyped(
  json: any,
  ignoreDiscriminator: boolean,
): UsersCreateTokenResponse {
  if (json == null) {
    return json;
  }
  return {};
}

export function UsersCreateTokenResponseToJSON(
  value?: UsersCreateTokenResponse | null,
): any {
  if (value == null) {
    return value;
  }
  return {};
}