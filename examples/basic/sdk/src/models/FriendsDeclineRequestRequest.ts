/* tslint:disable */
/* eslint-disable */
/**
 * Rivet SDK
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { mapValues } from '../runtime';
/**
 * 
 * @export
 * @interface FriendsDeclineRequestRequest
 */
export interface FriendsDeclineRequestRequest {
    /**
     * 
     * @type {string}
     * @memberof FriendsDeclineRequestRequest
     */
    userToken: string;
    /**
     * 
     * @type {string}
     * @memberof FriendsDeclineRequestRequest
     */
    friendRequestId: string;
}

/**
 * Check if a given object implements the FriendsDeclineRequestRequest interface.
 */
export function instanceOfFriendsDeclineRequestRequest(value: object): value is FriendsDeclineRequestRequest {
    if (!('userToken' in value) || value['userToken'] === undefined) return false;
    if (!('friendRequestId' in value) || value['friendRequestId'] === undefined) return false;
    return true;
}

export function FriendsDeclineRequestRequestFromJSON(json: any): FriendsDeclineRequestRequest {
    return FriendsDeclineRequestRequestFromJSONTyped(json, false);
}

export function FriendsDeclineRequestRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): FriendsDeclineRequestRequest {
    if (json == null) {
        return json;
    }
    return {
        
        'userToken': json['userToken'],
        'friendRequestId': json['friendRequestId'],
    };
}

export function FriendsDeclineRequestRequestToJSON(value?: FriendsDeclineRequestRequest | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'userToken': value['userToken'],
        'friendRequestId': value['friendRequestId'],
    };
}
