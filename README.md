This was an experiment that didn't end up working.  Maybe it will work one day as tooling becomes more capable and/or someone explains to me what I'm missing.

The idea was to demonstrate using [WAC](https://github.com/peterhuene/wac) to compose the following components:

- `service`: a component that exports a `handle` function which refers to imported resource types
- `virt`: a virtualizing component that both imports and exports those resource types
- `wrapper`: (intended to be) an "onion" wrapper around the above components, implementing the same world as `service`

The problem is that, AFAICT, there's no way to tell WAC that `wrapper` needs to export a `handle` function that refers to the host versions of the resource types rather than the virtualized versions.  Or perhaps it's more of a WIT limitation: there's no way to express that `wrapper` must refer to both the virtualized versions (internally) and host versions (externally) of the resource types.
