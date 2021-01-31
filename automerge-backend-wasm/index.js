import { State } from "./pkg/index.js"

/**
 * @typedef {Object} Backend
 * @property {State} state
 * @property {any[]} heads
 * @property {boolean} frozen
 */

/**
 * @returns {Backend}
 */
export function init() {
  return { state: State.new(), heads: [], frozen: false }
}

/**
 * @template Data
 * @param {Data} data
 * @returns {Backend}
 */
export function load(data) {
  const state = State.load(data)
  const heads = state.getHeads()
  return { state, heads, frozen: false }
}

/**
 * @param {Backend} backend
 * @returns {State}
 */
function backendState(backend) {
  if (backend.frozen) {
    throw new Error(
      "Attempting to use an outdated Automerge document that has already been updated. " +
        "Please use the latest document state, or call Automerge.clone() if you really " +
        "need to use this old document state."
    )
  }
  return backend.state
}

/**
 * @param {Backend} backend
 * @returns {Backend}
 */
export function clone(backend) {
  const state = backend.state.clone()
  return { state, heads: backend.heads.slice(), frozen: false }
}

/**
 * @param {Backend} backend
 * @returns {void}
 */
export function free(backend) {
  backend.state.free()
  backend.state = null
  backend.frozen = true
}

/**
 * @param {Backend} backend
 * @param {any[]} changes
 * @returns {[Backend, any]}
 */
export function applyChanges(backend, changes) {
  const state = backendState(backend)
  const [patch, heads] = state.applyChanges(changes)
  backend.frozen = true
  return [{ state, heads, frozen: false }, patch]
}

/**
 * @param {Backend} backend
 * @param {any} request
 * @returns {[Backend, any, any]}
 */
export function applyLocalChange(backend, request) {
  const state = backendState(backend)
  const [patch, change, heads] = state.applyLocalChange(request)
  backend.frozen = true
  return [{ state, heads, frozen: false }, patch, change]
}

/**
 * @param {Backend} backend
 * @param {any[]} changes
 * @returns {Backend}
 */
export function loadChanges(backend, changes) {
  const state = backendState(backend)
  const heads = state.loadChanges(changes)
  backend.frozen = true
  return { state, heads, frozen: false }
}

/**
 * @param {Backend} backend
 * @returns {any}
 */
export function getPatch(backend) {
  return backendState(backend).getPatch()
}

/**
 * @param {Backend} backend
 * @param {any} clock
 * @returns {any[]}
 */
export function getChanges(backend, clock) {
  return backendState(backend).getChanges(clock)
}

/**
 * @param {Backend} backend
 * @param {any} actor
 * @returns {any[]}
 */
export function getChangesForActor(backend, actor) {
  return backendState(backend).getChangesForActor(actor)
}

/**
 * @param {Backend} backend
 * @returns {any}
 */
export function getMissingDeps(backend) {
  return backendState(backend).getMissingDeps()
}

/**
 * @param {Backend} backend
 * @returns {Uint8Array}
 */
export function save(backend) {
  return backendState(backend).save()
}

/**
 * @param {Backend} backend
 * @returns {any[]}
 */
export function getHeads(backend) {
  return backend.heads
}
