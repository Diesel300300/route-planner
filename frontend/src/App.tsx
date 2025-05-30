import { useState, useEffect, useMemo } from 'react';
import { MapView } from './components/MapView';
import { PathInput } from './components/PathInput';
import { MapControl } from './components/MapControl';
import { fetchWays, assignColorsPaths, assignColorsWays } from './util/map';
import type { Path, Way } from './models/map';


function App() {
    const [markers, setMarkers] = useState<{ lat: number, lon: number}[]>([]);
    const [distance, setDistance] = useState<number>(500);
    const [amountPaths, setAmountPaths] = useState<number>(5);
    const [ways, setWays] = useState<Way[]>([]);
    const [paths, setPaths] = useState<Path[]>([]);
    const [nodesOn, setNodesOn] = useState<boolean>(false);
    const [waysOn, setWaysOn] = useState<boolean>(false);
    const [visiblePaths, setVisiblePaths] = useState<string[]>([]);
    
    useEffect(() => {
        fetchWays()
            .then(setWays)
            .catch(console.error);
    }, []);

  const colorMapWays = useMemo(
    () => assignColorsWays(ways),
    [ways]
  );

  const colorMapPaths = useMemo(
    () => assignColorsPaths(paths),
    [paths]
  );

    return (
        <div className="p-4">
            <div className="grid grid-rows-10 items-center">
                <h1 className="row-span-1 h-10 text-center text-2xl font-semibold">My Map Application</h1>
                <div className="row-span-9 grid grid-cols-6 gap-4 bg-gray-400">

                    <div className="col-span-1 p-4 ">
                        <div className='grid grid-rows-2'>
                            <div className='row-span-1'>
                                <PathInput
                                    markers={markers}
                                    distance={distance}
                                    setDistance={setDistance}
                                    amountPaths={amountPaths}
                                    setAmountPaths={setAmountPaths}
                                    setPaths={setPaths}
                                />
                            </div>
                            <div className='row-span-1 mt-4'>
                                Map Controls
                                <MapControl 
                                    nodesOn={nodesOn} 
                                    setNodesOn={setNodesOn}
                                    waysOn={waysOn}
                                    setWaysOn={setWaysOn}
                                    paths={paths}
                                    visiblePaths={visiblePaths}
                                    setVisiblePaths={setVisiblePaths}
                                />
                            </div>
                        </div>
                    </div>

                    <div className='col-span-5 h-screen p-1 space-y-4 bg-gray-200'>
                        <MapView 
                            ways={ways}
                            markers={markers} 
                            onMarkersChange={setMarkers} 
                            paths={paths} 
                            colorMapPaths={colorMapPaths}
                            colorMapWays={colorMapWays}
                            nodesOn={nodesOn}
                            waysOn={waysOn}
                            visiblePaths={visiblePaths}
                        />
                    </div>
                </div>
            </div>
        </div>
    )
}

export default App
