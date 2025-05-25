
export interface Node {
    id: string;
    lat: number;
    lon: number;
}

export interface Way {
    id: string;
    nodes: Node[];
}

export interface Path {
    id: string;
    distance: number;
    nodes: Node[];
}
