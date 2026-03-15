import { describe, it } from "node:test";
import * as assert from "node:assert/strict";
import {
  TemporalLayer,
  LayerMutability,
  layerName,
  layerFunction,
  layerMutability,
  layerDependsOn,
  allLayers,
} from "../src/layers";

describe("TemporalLayer", () => {
  it("five layers", () => {
    assert.equal(allLayers().length, 5);
  });

  it("epoch is 0", () => {
    assert.equal(TemporalLayer.Epoch, 0);
  });

  it("display is 4", () => {
    assert.equal(TemporalLayer.Display, 4);
  });

  it("names", () => {
    assert.equal(layerName(TemporalLayer.Epoch), "Epoch");
    assert.equal(layerName(TemporalLayer.Quant), "Quant");
    assert.equal(layerName(TemporalLayer.UniversalDuration), "Universal Duration");
    assert.equal(layerName(TemporalLayer.SolarClock), "Solar Clock");
    assert.equal(layerName(TemporalLayer.Display), "Display");
  });

  it("functions non-empty", () => {
    for (const layer of allLayers()) {
      assert.ok(layerFunction(layer).length > 0);
    }
  });

  it("mutability", () => {
    assert.equal(layerMutability(TemporalLayer.Epoch), LayerMutability.Immutable);
    assert.equal(layerMutability(TemporalLayer.Quant), LayerMutability.ImmutablePhysics);
    assert.equal(layerMutability(TemporalLayer.Display), LayerMutability.Customizable);
  });

  it("depends on lower layers", () => {
    assert.ok(layerDependsOn(TemporalLayer.Quant, TemporalLayer.Epoch));
    assert.ok(layerDependsOn(TemporalLayer.Display, TemporalLayer.Epoch));
    assert.ok(!layerDependsOn(TemporalLayer.Epoch, TemporalLayer.Quant));
  });

  it("layer does not depend on itself", () => {
    assert.ok(!layerDependsOn(TemporalLayer.Epoch, TemporalLayer.Epoch));
  });
});
