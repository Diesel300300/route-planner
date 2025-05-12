import { useEffect, useState } from 'react';
import { MapView } from './MapView'

import {setup, parse_osm_ways_with_tags, Node } from '../pkg/route_parser.js';

const accepted_road_types = [
    "residential",
    "unclassified",
    "track",
    "service",
    "tertiary",
    "road",
    "secondary",
    "primary",
    "trunk",
    "primary_link",
    "trunk_link",
    "tertiary_link",
    "secondary_link",
    "highway",
]

function App() {
    const [nodes, setNodes] = useState<Node[]>([]);
    useEffect(() => {
        (async () => {
            setup();
            const osmXml = await fetch('/data/map').then(r => r.text());
            const wasmRoads: Node[] = Array.from(parse_osm_ways_with_tags(osmXml, accepted_road_types));
            console.log("Parsed nodes:", nodes.length, nodes.slice(0,5));
            setNodes(wasmRoads);
        })();
    });

    return (
        <div className='h-screen p-4 space-y-4 bg-gray-200'>
            <h1 className="text-2xl font-semibold">My Map Application</h1>

            <MapView nodes={nodes}/>
        </div>
    )
}

export default App
