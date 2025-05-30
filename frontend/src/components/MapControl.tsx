import type {Path} from '../models/map';



interface MapControlProps {
    nodesOn: boolean;
    setNodesOn: (nodesOn: boolean) => void;
    waysOn: boolean;
    setWaysOn: (waysOn: boolean) => void;
    paths: Path[];
    visiblePaths: string[];
    setVisiblePaths: (visiblePaths: string[]) => void;
}

export function MapControl({ nodesOn, setNodesOn, waysOn, setWaysOn, paths, visiblePaths, setVisiblePaths }: MapControlProps) {
    return (
        <div className="p-4 bg-white rounded-lg shadow-md">
            <div className="">
                <div>
                    <h2 className="text-lg font-semibold mb-2">Routes</h2>                   
                    {!paths?.length ? ( 
                        <p className="text-gray-500">No paths available.</p>
                    ) : (paths.map((path, i) => {
                            const isVisible = visiblePaths.includes(path.id);
                            return (
                                <div key={path.id} className="flex items-center mb-2">
                                    <input
                                        type="checkbox"
                                        className="mr-2"
                                        checked={isVisible}
                                        onChange={(e) => {
                                            if (e.target.checked) {
                                                setVisiblePaths([...visiblePaths, path.id]);
                                            } else {
                                                setVisiblePaths(visiblePaths.filter((id) => id !== path.id));
                                            }
                                        }}
                                    />
                                    <div className="flex-1">
                                        <div className="font-medium">Route {i + 1}</div>
                                        <div className="text-sm text-gray-600">
                                            Distance: {path.distance.toFixed(1)} m
                                        </div>
                                    </div>
                                </div>
                            );

                        }))
                    }

                </div>

                <label className="flex items-center">
                    <input
                        type="checkbox"
                        checked={nodesOn}
                        onChange={(e) => setNodesOn(e.target.checked)}
                        className="mr-2"
                    />
                    Show Nodes
                </label>
                <label className="flex items-center mt-2">
                    <input
                        type="checkbox"
                        checked={waysOn}
                        onChange={(e) => setWaysOn(e.target.checked)}
                        className="mr-2"
                    />
                    Show Ways
                </label>
            </div>
        </div>
    );
    
}
