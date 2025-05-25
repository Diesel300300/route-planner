


interface MapControlProps {
    nodesOn: boolean;
    setNodesOn: (nodesOn: boolean) => void;
    waysOn: boolean;
    setWaysOn: (waysOn: boolean) => void;
}

export function MapControl({ nodesOn, setNodesOn, waysOn, setWaysOn }: MapControlProps) {
    return (
        <div className="p-4 bg-white rounded-lg shadow-md">
            <div className="">
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
