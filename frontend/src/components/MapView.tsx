import { useMemo, useState, useRef } from 'react';
import Map, { Source, Layer, Popup, Marker } from 'react-map-gl/maplibre';
import type { MapLayerMouseEvent } from 'react-map-gl/maplibre';
import 'maplibre-gl/dist/maplibre-gl.css';

import type { Path, Way } from '../models/map';


interface MapViewProps {
    ways: Way[];
    markers: { lat: number; lon: number }[];
    onMarkersChange: (m: {lat: number, lon: number}[]) => void;
    paths: Path[];
    colorMapPaths: Map<string, string>;
    colorMapWays: Map<string, string>;
    nodesOn: boolean;
    waysOn: boolean;
    visiblePaths: string[];
}

export function MapView({ ways, markers, onMarkersChange, paths, colorMapPaths,colorMapWays, nodesOn, waysOn, visiblePaths }: MapViewProps ) {
    if (!ways?.length) {
        return <div className="flex items-center justify-center h-full">Loading...</div>;
    }

    const linesGeo = useMemo<GeoJSON.FeatureCollection>(() => ({
        type: 'FeatureCollection',
        features: paths?.map((path) => ({
            type: 'Feature',
            geometry: {
                type: 'LineString',
                coordinates: path.nodes.map((n) => [n.lon, n.lat]),
            },
            properties: {
                id:       path.id,
                color:    colorMapPaths.get(path.id) ?? '#000',
                distance: path.distance,
            },
        })),
    }), [paths, colorMapPaths]);
    


    const wayLinesGeo = useMemo<GeoJSON.FeatureCollection>(() => ({
        type: 'FeatureCollection',
        features: ways.map((way): GeoJSON.Feature => ({
            type: 'Feature',
            geometry: {
                type: 'LineString',
                coordinates: way.nodes.map(n => [n.lon, n.lat]),
            },
            properties: {
                id:    way.id,
                color: colorMapWays.get(way.id) || '#000000',
            },
        })),
    }), [ways, colorMapPaths]);

    const nodesGeo = useMemo<GeoJSON.FeatureCollection>(() => ({
    type: 'FeatureCollection',
    features: ways.flatMap((way) =>
      way.nodes.map((n): GeoJSON.Feature => ({
        type: 'Feature',
        geometry: { type: 'Point', coordinates: [n.lon, n.lat] },
        properties: { id: n.id, wayId: way.id, color: colorMapWays.get(way.id) }
      }))
    )
  }), [ways, colorMapPaths]);

    const [hoverInfo, setHoverInfo] = useState<{
        lngLat: { lat: number; lng: number };
        id: string;
        distance: number;
    } | null>(null);

    const mapRef = useRef<maplibregl.Map | null>(null);

    const handleMapLeftClick = (e: maplibregl.MapLayerMouseEvent & { originalEvent: MouseEvent}) => {
        if (e.originalEvent.button !== 0) return;  
        if (markers.length >= 2) return; 
        onMarkersChange([
            ...markers,
            { lat: e.lngLat.lat, lon: e.lngLat.lng }

        ]);
        console.log('Marker added:', e.lngLat.lat, e.lngLat.lng);
    };

    const handleMapRightClick = (e: maplibregl.MapLayerMouseEvent & { originalEvent: MouseEvent}) => {
        e.originalEvent.preventDefault();
        onMarkersChange(markers.slice(0, -1));
    };

    const pathFilter = useMemo<any>(() => {
        return ["in", "id", ...visiblePaths];
    }, [visiblePaths]);

    return (
        <Map
            ref={(el) => { mapRef.current = el?.getMap() ?? null; }}
            
            initialViewState={{
                latitude: 51.069144704301806,
                longitude: 4.038468861183304,
                zoom:      14,
            }}
            
            style={{ width: '100%', height: '100%' }}
            mapStyle="https://api.maptiler.com/maps/basic-v2/style.json?key=4LPNKFS9ll0O8UdyvdYy"
            interactiveLayerIds={['paths-hit']} 
            onMouseMove={(e: MapLayerMouseEvent) => {
                const feature = e.features?.[0];
                if (feature && feature.properties) {
                    setHoverInfo({
                        lngLat: e.lngLat,
                        id: feature.properties.id,
                        distance: feature.properties.distance || 0,
                    });
                } else {
                    setHoverInfo(null);
                }
            }}
            onMouseLeave={() => setHoverInfo(null)}
            onClick={handleMapLeftClick}
            onContextMenu={handleMapRightClick}
        >

            <Source id="paths" type="geojson" data={linesGeo}>
                {/* only these two filtered layers, drop the old ones */}
                <Layer
                    id="paths-layer"
                    type="line"
                    filter={pathFilter}
                    paint={{
                        "line-color": ["get", "color"],
                        "line-width": 2,
                    }}
                />
                <Layer
                    id="paths-hit"
                    type="line"
                    filter={pathFilter}
                    paint={{
                        "line-color": "#000",
                        "line-width": 12,
                        "line-opacity": 0,
                    }}
                />
            </Source>

            { /*paths && paths.length > 0 && visiblePaths.length > 0 &&  

             <Source id="paths" type="geojson" data={linesGeo}>
                <Layer id="paths-layer" type="line" paint={{
                    'line-color': ['get','color'],
                    'line-width': 2
                }} />
                <Layer id="paths-hit" type="line" paint={{
                    'line-color': '#000',
                    'line-width': 12,
                    'line-opacity': 0
                }} />
            </Source>
            */}
            
            {nodesOn && (
            <Source id="nodes" type="geojson" data={nodesGeo}>
                <Layer id="nodes-layer" type="circle" paint={{
                    'circle-radius': 3,
                    'circle-color': ['get', 'color'],
                }} />
            </Source>
            )}

            {waysOn && (
            <Source id="way-lines" type="geojson" data={wayLinesGeo}>
                <Layer id="way-lines-layer" type="line" paint={{
                    'line-color': ['get', 'color'],
                    'line-width': 2,
                }} />
            </Source>
            )}

            {hoverInfo && (
                <Popup
                    longitude={hoverInfo.lngLat.lng}
                    latitude={hoverInfo.lngLat.lat}
                    anchor="bottom"
                    closeButton={false}
                    closeOnClick={false}
                >
                    <div className="grid grid-cols-1">
                        <div> Path ID: {hoverInfo.id} </div>
                        <div> Distance: {hoverInfo.distance} m </div>
                    </div>

                </Popup>
            )}

            {markers.map((m, i) => (
                <Marker
                    key={i}
                    longitude={m.lon}
                    latitude={m.lat}
                    anchor="center"
                >
                    <div className="flex flex-col items-center">
                        {/* colored dot */}
                        <div
                            className={`w-4 h-4 rounded-full border-2 border-white ${i === 0 ? 'bg-green-500' : 'bg-red-500'}`}
                        />
                        {/* label */}
                        <span className="mt-1 px-1 text-xs font-semibold text-white 
                            bg-black bg-opacity-50 rounded">
                            {i === 0 ? 'Start' : 'Goal'}
                        </span>
                    </div>
                </Marker>
            ))}
        </Map>
    );
}

