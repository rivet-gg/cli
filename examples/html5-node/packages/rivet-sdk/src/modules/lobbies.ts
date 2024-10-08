// This file is auto-generated by the Rivet (https://rivet.gg) build system.
//
// Do not edit this file directly.

import { Client } from "../client/client.ts";

/**
 * Lobbies
 *
 * Lobby & player management. Create & join lobbies instantly.
 */
export class RivetLobbies {
  client: Client;

  constructor(client: Client) {
    this.client = client;
  }

  /**
   * Create Lobby
   * Creates a new lobby on-demand.
   */
  create(body: any = {}): Promise<any> {
    return this.client.buildRequest(
      "lobbies.create",
      "POST",
      "/modules/lobbies/scripts/create/call",
      body,
    );
  }

  /**
   * Destroy Lobby
   * Destroys an existing lobby.
   */
  destroy(body: any = {}): Promise<any> {
    return this.client.buildRequest(
      "lobbies.destroy",
      "POST",
      "/modules/lobbies/scripts/destroy/call",
      body,
    );
  }

  /**
   * Find Or Create Lobby
   * Finds a lobby or creates one if there are no available spots for players.
   */
  findOrCreate(body: any = {}): Promise<any> {
    return this.client.buildRequest(
      "lobbies.find_or_create",
      "POST",
      "/modules/lobbies/scripts/find_or_create/call",
      body,
    );
  }

  /**
   * Join Lobby
   * Add a player to an existing lobby.
   */
  join(body: any = {}): Promise<any> {
    return this.client.buildRequest(
      "lobbies.join",
      "POST",
      "/modules/lobbies/scripts/join/call",
      body,
    );
  }

  /**
   * List Lobbies
   * List & query all lobbies.
   */
  list(body: any = {}): Promise<any> {
    return this.client.buildRequest(
      "lobbies.list",
      "POST",
      "/modules/lobbies/scripts/list/call",
      body,
    );
  }

  /**
   * Set Lobby Ready
   * Called on lobby startup after initiation to notify it can start accepting player. This should be called after operations like loading maps are complete.
   */
  setLobbyReady(body: any = {}): Promise<any> {
    return this.client.buildRequest(
      "lobbies.set_lobby_ready",
      "POST",
      "/modules/lobbies/scripts/set_lobby_ready/call",
      body,
    );
  }

  /**
   * Set Player Connected
   * Called when a player connects to the lobby.
   */
  setPlayerConnected(body: any = {}): Promise<any> {
    return this.client.buildRequest(
      "lobbies.set_player_connected",
      "POST",
      "/modules/lobbies/scripts/set_player_connected/call",
      body,
    );
  }

  /**
   * Set Player Disconnected
   * Called when a player disconnects from the lobby.
   */
  setPlayerDisconnected(body: any = {}): Promise<any> {
    return this.client.buildRequest(
      "lobbies.set_player_disconnected",
      "POST",
      "/modules/lobbies/scripts/set_player_disconnected/call",
      body,
    );
  }

  /**
   * Find Lobby
   * Finds an existing lobby with a given query. This will not create a new lobby, see `find_or_create` instead.
   */
  find(body: any = {}): Promise<any> {
    return this.client.buildRequest(
      "lobbies.find",
      "POST",
      "/modules/lobbies/scripts/find/call",
      body,
    );
  }

  /**
   * List Regions
   * List available regions.
   */
  listRegions(body: any = {}): Promise<any> {
    return this.client.buildRequest(
      "lobbies.list_regions",
      "POST",
      "/modules/lobbies/scripts/list_regions/call",
      body,
    );
  }

  /**
   * Force Garbage Collection
   * Rarely used. Forces the matchmaker to purge lobbies & players.
   */
  forceGc(body: any = {}): Promise<any> {
    return this.client.buildRequest(
      "lobbies.force_gc",
      "POST",
      "/modules/lobbies/scripts/force_gc/call",
      body,
    );
  }

  /**
   * Fetch Lobby Manager State
   * See full state of the lobby manager for debugging.
   */
  fetchLobbyManagerState(body: any = {}): Promise<any> {
    return this.client.buildRequest(
      "lobbies.fetch_lobby_manager_state",
      "POST",
      "/modules/lobbies/scripts/fetch_lobby_manager_state/call",
      body,
    );
  }

  /**
   * Reset Lobby Manager State
   * Reset lobby manager state. For debugging only.
   */
  resetLobbyManagerState(body: any = {}): Promise<any> {
    return this.client.buildRequest(
      "lobbies.reset_lobby_manager_state",
      "POST",
      "/modules/lobbies/scripts/reset_lobby_manager_state/call",
      body,
    );
  }
}
