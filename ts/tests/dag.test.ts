import { describe, it } from "node:test";
import * as assert from "node:assert/strict";
import { TemporalNode, TemporalDag } from "../src/dag";
import { Quant } from "../src/quant";
import { DagError } from "../src/error";

describe("TemporalNode", () => {
  it("genesis", () => {
    const node = TemporalNode.genesis("g0");
    assert.equal(node.id, "g0");
    assert.equal(node.timestamp.value, 0n);
    assert.deepEqual(node.parents, []);
    assert.ok(node.isCheckpoint);
    assert.ok(node.isRoot());
    assert.ok(!node.isMerge());
  });

  it("checkpoint", () => {
    const node = TemporalNode.checkpoint("cp1", new Quant(100n), ["g0"]);
    assert.ok(node.isCheckpoint);
    assert.deepEqual(node.parents, ["g0"]);
  });

  it("regular node", () => {
    const node = new TemporalNode("n1", new Quant(50n), ["g0"]);
    assert.ok(!node.isCheckpoint);
    assert.ok(!node.isRoot());
    assert.equal(node.payload, null);
  });

  it("merge node", () => {
    const node = new TemporalNode("m1", new Quant(200n), ["n1", "n2"]);
    assert.ok(node.isMerge());
  });
});

describe("TemporalDag", () => {
  it("add genesis and node", () => {
    const dag = new TemporalDag();
    dag.addNode(TemporalNode.genesis("g0"));
    dag.addNode(new TemporalNode("n1", new Quant(10n), ["g0"]));
    assert.equal(dag.nodeCount(), 2);
  });

  it("get node", () => {
    const dag = new TemporalDag();
    dag.addNode(TemporalNode.genesis("g0"));
    const node = dag.getNode("g0");
    assert.notEqual(node, undefined);
    assert.equal(node!.id, "g0");
  });

  it("get nonexistent node", () => {
    const dag = new TemporalDag();
    assert.equal(dag.getNode("nope"), undefined);
  });

  it("node ids", () => {
    const dag = new TemporalDag();
    dag.addNode(TemporalNode.genesis("g0"));
    dag.addNode(new TemporalNode("n1", new Quant(10n), ["g0"]));
    const ids = dag.nodeIds();
    assert.ok(ids.includes("g0"));
    assert.ok(ids.includes("n1"));
  });

  it("duplicate node throws", () => {
    const dag = new TemporalDag();
    dag.addNode(TemporalNode.genesis("g0"));
    assert.throws(() => dag.addNode(TemporalNode.genesis("g0")), DagError);
  });

  it("missing parent throws", () => {
    const dag = new TemporalDag();
    assert.throws(
      () => dag.addNode(new TemporalNode("n1", new Quant(10n), ["missing"])),
      DagError,
    );
  });

  it("causal violation throws", () => {
    const dag = new TemporalDag();
    dag.addNode(TemporalNode.genesis("g0"));
    dag.addNode(new TemporalNode("n1", new Quant(100n), ["g0"]));
    assert.throws(
      () => dag.addNode(new TemporalNode("n2", new Quant(50n), ["n1"])),
      DagError,
    );
  });

  it("edges", () => {
    const dag = new TemporalDag();
    dag.addNode(TemporalNode.genesis("g0"));
    dag.addNode(new TemporalNode("n1", new Quant(10n), ["g0"]));
    dag.addNode(new TemporalNode("n2", new Quant(20n), ["n1"]));
    const edges = dag.edges();
    assert.equal(edges.length, 2);
  });

  it("checkpoints", () => {
    const dag = new TemporalDag();
    dag.addNode(TemporalNode.genesis("g0"));
    dag.addNode(new TemporalNode("n1", new Quant(10n), ["g0"]));
    dag.addNode(TemporalNode.checkpoint("cp1", new Quant(20n), ["n1"]));
    assert.equal(dag.checkpoints().length, 2);
  });

  it("topological sort", () => {
    const dag = new TemporalDag();
    dag.addNode(TemporalNode.genesis("g0"));
    dag.addNode(new TemporalNode("n1", new Quant(20n), ["g0"]));
    dag.addNode(new TemporalNode("n2", new Quant(10n), ["g0"]));
    const sorted = dag.topologicalSort();
    assert.equal(sorted[0].id, "g0");
    assert.ok(sorted[1].timestamp.value <= sorted[2].timestamp.value);
  });

  it("prune node", () => {
    const dag = new TemporalDag();
    dag.addNode(TemporalNode.genesis("g0"));
    dag.addNode(new TemporalNode("n1", new Quant(10n), ["g0"]));
    dag.pruneNode("n1");
    assert.ok(dag.getNode("n1")!.isPruned);
  });

  it("cannot prune checkpoint", () => {
    const dag = new TemporalDag();
    dag.addNode(TemporalNode.genesis("g0"));
    assert.throws(() => dag.pruneNode("g0"), DagError);
  });

  it("prune nonexistent throws", () => {
    const dag = new TemporalDag();
    assert.throws(() => dag.pruneNode("nope"), DagError);
  });

  it("merge node", () => {
    const dag = new TemporalDag();
    dag.addNode(TemporalNode.genesis("g0"));
    dag.addNode(new TemporalNode("n1", new Quant(10n), ["g0"]));
    dag.addNode(new TemporalNode("n2", new Quant(20n), ["g0"]));
    dag.addNode(new TemporalNode("m1", new Quant(30n), ["n1", "n2"]));
    assert.ok(dag.getNode("m1")!.isMerge());
    assert.equal(dag.nodeCount(), 4);
  });
});
