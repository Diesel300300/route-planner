
interface PathInputProps {
    markers: { lat: number; lon: number }[];
    distance: number;
    setDistance: (distance: number) => void;
    amountPaths: number;
    setAmountPaths: (amount: number) => void;
    fetchPaths: (start: { lat: number; lon: number }, end: { lat: number; lon: number }, distance: number, amountPaths: number) => Promise<any[]>;
    setPaths: (paths: any[]) => void;
}


export function PathInput({ markers, distance, setDistance, amountPaths, setAmountPaths, fetchPaths, setPaths }: PathInputProps) {

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
            <button
                className='mt-2 w-full p-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600'
                onClick={() => 
                    {
                        if (markers.length < 2) {
                            // TODO: nice error handling
                            alert('Please select two markers on the map.');
                            return;
                        }
                        fetchPaths(markers[0], markers[1], distance, amountPaths)
                        .then(setPaths)
                        .catch(console.error)
                    }
                }
            > Calculate Route
            </button>
        </div>
    )
}
