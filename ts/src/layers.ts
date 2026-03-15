/**
 * The 5-layer temporal architecture.
 *
 * Each layer depends only on lower layers.
 */
export enum TemporalLayer {
  Epoch = 0,
  Quant = 1,
  UniversalDuration = 2,
  SolarClock = 3,
  Display = 4,
}

export enum LayerMutability {
  Immutable = "Immutable",
  ImmutablePhysics = "Immutable (physics)",
  PerPlanetProfile = "Per planet profile",
  Customizable = "Customizable",
}

export function layerName(layer: TemporalLayer): string {
  switch (layer) {
    case TemporalLayer.Epoch: return "Epoch";
    case TemporalLayer.Quant: return "Quant";
    case TemporalLayer.UniversalDuration: return "Universal Duration";
    case TemporalLayer.SolarClock: return "Solar Clock";
    case TemporalLayer.Display: return "Display";
  }
}

export function layerFunction(layer: TemporalLayer): string {
  switch (layer) {
    case TemporalLayer.Epoch: return "Shared origin point (t:0)";
    case TemporalLayer.Quant: return "Physical substrate (hydrogen hyperfine periods)";
    case TemporalLayer.UniversalDuration: return "Planet-independent elapsed-time units";
    case TemporalLayer.SolarClock: return "Planet-specific position-in-day coordinates";
    case TemporalLayer.Display: return "Human-facing rendering (clock faces, calendars, etc.)";
  }
}

export function layerMutability(layer: TemporalLayer): LayerMutability {
  switch (layer) {
    case TemporalLayer.Epoch: return LayerMutability.Immutable;
    case TemporalLayer.Quant: return LayerMutability.ImmutablePhysics;
    case TemporalLayer.UniversalDuration: return LayerMutability.Immutable;
    case TemporalLayer.SolarClock: return LayerMutability.PerPlanetProfile;
    case TemporalLayer.Display: return LayerMutability.Customizable;
  }
}

export function layerDependsOn(layer: TemporalLayer, other: TemporalLayer): boolean {
  return other < layer;
}

export function allLayers(): TemporalLayer[] {
  return [
    TemporalLayer.Epoch,
    TemporalLayer.Quant,
    TemporalLayer.UniversalDuration,
    TemporalLayer.SolarClock,
    TemporalLayer.Display,
  ];
}
