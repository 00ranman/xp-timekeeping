import { DagError } from "./error";
import { Quant } from "./quant";

/** A node in the temporal DAG (Directed Acyclic Graph). */
export class TemporalNode {
  readonly id: string;
  readonly timestamp: Quant;
  readonly parents: string[];
  isCheckpoint: boolean;
  isPruned: boolean;
  payload: string | null;

  constructor(id: string, timestamp: Quant, parents: string[]) {
    this.id = id;
    this.timestamp = timestamp;
    this.parents = parents;
    this.isCheckpoint = false;
    this.isPruned = false;
    this.payload = null;
  }

  static genesis(id: string): TemporalNode {
    const node = new TemporalNode(id, Quant.zero(), []);
    node.isCheckpoint = true;
    return node;
  }

  static checkpoint(id: string, timestamp: Quant, parents: string[]): TemporalNode {
    const node = new TemporalNode(id, timestamp, parents);
    node.isCheckpoint = true;
    return node;
  }

  isMerge(): boolean {
    return this.parents.length > 1;
  }

  isRoot(): boolean {
    return this.parents.length === 0;
  }
}

/** An edge in the temporal DAG, representing a causal relationship. */
export interface TemporalEdge {
  from: string;
  to: string;
  quantDelta: Quant;
}

/** The temporal DAG substrate. */
export class TemporalDag {
  private nodes = new Map<string, TemporalNode>();

  addNode(node: TemporalNode): void {
    for (const parentId of node.parents) {
      const parent = this.nodes.get(parentId);
      if (!parent) {
        throw new DagError(`parent node '${parentId}' not found`);
      }
      if (parent.timestamp.value >= node.timestamp.value) {
        throw new DagError(
          `causal violation: parent '${parentId}' timestamp ${parent.timestamp} >= node '${node.id}' timestamp ${node.timestamp}`
        );
      }
    }

    if (this.nodes.has(node.id)) {
      throw new DagError(`node '${node.id}' already exists`);
    }

    this.nodes.set(node.id, node);
  }

  getNode(id: string): TemporalNode | undefined {
    return this.nodes.get(id);
  }

  nodeIds(): string[] {
    return Array.from(this.nodes.keys());
  }

  nodeCount(): number {
    return this.nodes.size;
  }

  checkpoints(): TemporalNode[] {
    return Array.from(this.nodes.values()).filter((n) => n.isCheckpoint);
  }

  edges(): TemporalEdge[] {
    const result: TemporalEdge[] = [];
    for (const node of this.nodes.values()) {
      for (const parentId of node.parents) {
        const parent = this.nodes.get(parentId);
        if (parent) {
          result.push({
            from: parentId,
            to: node.id,
            quantDelta: node.timestamp.absDiff(parent.timestamp),
          });
        }
      }
    }
    return result;
  }

  topologicalSort(): TemporalNode[] {
    const sorted = Array.from(this.nodes.values());
    sorted.sort((a, b) => {
      if (a.timestamp.value < b.timestamp.value) return -1;
      if (a.timestamp.value > b.timestamp.value) return 1;
      return 0;
    });
    return sorted;
  }

  pruneNode(id: string): void {
    const node = this.nodes.get(id);
    if (!node) {
      throw new DagError(`node '${id}' not found`);
    }
    if (node.isCheckpoint) {
      throw new DagError(`cannot prune checkpoint node '${id}'`);
    }
    node.isPruned = true;
  }
}
