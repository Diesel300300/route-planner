import { useEffect, useState } from 'react';
import { MapView } from './MapView'

import {setup, parse_osm_roads, Node } from '../pkg/core.js';


function App() {
    const [nodes, setNodes] = useState<Node[]>([]);
    useEffect(() => {
        (async () => {
            setup();
            const osmXml = await fetch('/data/map').then(r => r.text());
            const wasmRoads: Node[] = Array.from(parse_osm_roads(osmXml));
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
