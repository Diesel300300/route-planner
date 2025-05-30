import { useState } from 'react';
import {fetchPathsDfs, fetchPathsSpecialDijkstra, fetchPathsBfs} from '../util/map.ts';



interface PathInputProps {
    markers: { lat: number; lon: number }[];
    distance: number;
    setDistance: (distance: number) => void;
    amountPaths: number;
    setAmountPaths: (amount: number) => void;
    setPaths: (paths: any[]) => void;
}



export function PathInput({ markers, distance, setDistance, amountPaths, setAmountPaths, setPaths }: PathInputProps) {
    const [pathAlgorithm, setPathAlgorithm] = useState<string>('dfs');
    function handleCalculatePath() {
        if (markers.length < 2) {
            // TODO: nice error handling
            alert('Please select two markers on the map.');
            return;
        }
        if (pathAlgorithm === 'special_dijkstra') { 
            fetchPathsSpecialDijkstra(markers[0], markers[1], distance, amountPaths)
                .then(setPaths)
                .then(console.log)
                .catch(console.error);
        } else if (pathAlgorithm === 'dfs') {
            fetchPathsDfs(markers[0], markers[1], distance, amountPaths)
                .then(setPaths)
                .then(console.log)
                .catch(console.error);
        } else if (pathAlgorithm === 'bfs') {
            fetchPathsBfs(markers[0], markers[1], distance, amountPaths)
                .then(setPaths)
                .then(console.log)
                .catch(console.error)
        }
    }

    return (
        <div>
            <h2 className='text-xl font-semibold'> Menu</h2>
            <label className='block mt-2'>
                Distance:
                <input 
                    className='w-full p-1 border border-black rounded-lg'
                    type="number"
                    placeholder='Enter distance'
                    value={distance}
                    onChange={(e) => setDistance(Number(e.target.value))}
                />
            </label>
            <label className='block mt-2'>
                Amount of Paths:
                <input 
                    className='w-full p-1 border border-black rounded-lg'
                    type="number"
                    placeholder='Enter amount of paths'
                    value={amountPaths}
                    onChange={(e) => setAmountPaths(Number(e.target.value))}
                />
            </label>
            <label className='block mt-2'>
                Path Algorithm:
                <select 
                    className='w-full p-1 border border-black rounded-lg'
                    value={pathAlgorithm}
                    onChange={(e) => setPathAlgorithm(e.target.value)}
                >
                    <option value="dfs">DFS</option>
                    <option value="bfs">BFS</option>
                    <option value="special_dijkstra">Special Dijkstra</option>
                </select>
            </label>

            <button
                className='mt-2 w-full p-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600'
                onClick={handleCalculatePath}
            > Calculate Route
            </button>
        </div>
    )
}
