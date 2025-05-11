// src/MapView.tsx
import { useMemo, useState, useRef, useEffect } from 'react';
import Map, { Source, Layer, Popup } from 'react-map-gl/maplibre';
import type { MapLayerMouseEvent } from 'react-map-gl/maplibre';
import { Node } from '../pkg/core';
import 'maplibre-gl/dist/maplibre-gl.css';

export function MapView({ nodes }: { nodes: Node[] }) {
    const geojson = useMemo(() => ({
        type: 'FeatureCollection' as const,
        features: nodes.map(n => ({
            type: 'Feature' as const,
            geometry: {
                type: 'Point' as const,
                coordinates: [n.lon, n.lat],
            },
            properties: { id: Number(n.id) },
        })),
    }), [nodes]);

    const layerStyle = useMemo(() => ({
        id: 'nodes-layer',
        type: 'circle' as const,
        paint: {
            'circle-radius': 5,
            'circle-color': '#007cbf',
            'circle-opacity': 1,
        },
    }), []);

    const [hoverInfo, setHoverInfo] = useState<{
        id: number;
        lngLat: { lng: number; lat: number };
    } | null>(null);

    const mapRef = useRef<maplibregl.Map | null>(null);
    const [viewGeo, setViewGeo] = useState(geojson);

    const filterByViewport = () => {
        const map = mapRef.current;
        if (!map) return;
        const bounds = map.getBounds();
        const features = geojson.features.filter(f => {
            const [lng, lat] = (f.geometry as any).coordinates;
            return bounds.contains([lng, lat]);
        });
        setViewGeo({ type: 'FeatureCollection', features });
    };

    useEffect(() => {
        const map = mapRef.current;
        if (!map) return;

        // Bind and unbind moveend
        map.on('moveend', filterByViewport);
        // initial filter
        filterByViewport();

        return () => { map.off('moveend', filterByViewport); };
    }, [geojson]);


    return (
        <Map
            ref={(el) => { mapRef.current = el?.getMap() ?? null; }}
            initialViewState={{
                longitude: nodes[0]?.lon ?? 0,
                latitude:  nodes[0]?.lat ?? 0,
                zoom:      12,
            }}
            style={{ width: '100%', height: '100%' }}
            mapStyle="https://api.maptiler.com/maps/basic-v2/style.json?key=4LPNKFS9ll0O8UdyvdYy"
            interactiveLayerIds={['nodes-layer']} 
            onMouseMove={(e: MapLayerMouseEvent) => {
                const feature = e.features?.[0];
                if (feature && feature.properties) {
                    setHoverInfo({
                        id: Number(feature.properties.id),
                        lngLat: e.lngLat
                    });
                } else {
                    setHoverInfo(null);
                }
            }}
            onMouseLeave={() => setHoverInfo(null)}
        >
            <Source id="nodes" type="geojson" data={viewGeo}
                cluster={true}
                clusterMaxZoom={12} 
                clusterRadius={50} 
            >
                <Layer {...layerStyle} />
            </Source>

            {hoverInfo && (
                <Popup
                    longitude={hoverInfo.lngLat.lng}
                    latitude={hoverInfo.lngLat.lat}
                    anchor="bottom"
                    closeButton={false}
                    closeOnClick={false}
                >
                    <div className="grid grid-cols-1">
                        <div> Node ID: {hoverInfo.id} </div>
                        <div> lng: {hoverInfo.lngLat.lng.toFixed(4)} </div>
                        <div> lat: {hoverInfo.lngLat.lat.toFixed(4)} </div>
                    </div>

                </Popup>
            )}
        </Map>
    );
}

