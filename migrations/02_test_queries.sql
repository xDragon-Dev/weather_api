INSERT INTO users(id,name,email,pass_hash) VALUES
('5100e4e9-533f-43e9-8b25-c116dc4dfe9e', 'Juanito', 'juan@gmail.com','12345678'),
('557ccfcd-45c9-4ba1-a8a4-d8ea8688809b', 'Maria', 'maria@gmail.com', '12345678'),
('9e70e01b-e7c7-4abd-9994-7ce857dd9b5d', 'Pedrito', 'pedrito@gmail.com','12345678'),
('49e016e3-49ec-4c79-abb5-93360048055d', 'Pablito', 'pablito@gmail.com', '12345678');

INSERT INTO nodes(id, continent, country, region, city, district, lat, lon, alt_m, last_heartbeat) VALUES
(
    '1fd52926-3557-44ca-9f54-f4d6574d0cfa',
    'America',
    'Mexico',
    'Puebla',
    'Puebla',
    'Ciudad Universitaria',
    0.0,
    0.0,
    0.0,
    CURRENT_TIMESTAMP
),
(
    'f0e7b9fb-edd8-4062-9abf-bd90b82d9301',
    'America',
    'Mexico',
    'Puebla',
    'Puebla',
    'Ciudad Universitaria 2',
    0.0,
    0.0,
    0.0,
    CURRENT_TIMESTAMP
);


INSERT INTO logs(temp, hum, air_q, timestamp, node_id) VALUES
(0.0, 0.0, 0.0, CURRENT_TIMESTAMP, '1fd52926-3557-44ca-9f54-f4d6574d0cfa'),
(0.0, 0.0, 0.0, CURRENT_TIMESTAMP, 'f0e7b9fb-edd8-4062-9abf-bd90b82d9301');

/*
9895eb39-b562-4db5-a97e-dc92571be9e5
3f4c0b0f-cad9-4938-9dfe-0877cc8808df
7708aa1a-00ee-4869-9842-99209d50f7e1
2929e4b3-02d0-48c9-9812-1c1859ea1fda
c97da087-eac5-4016-b182-9e1596e6a9f6
40f35aef-5a89-4d3e-b2b0-d3eeb0c4ddd2
aa24c2ae-33a6-4fb8-81af-60d610c9f76a
18e911d7-32dd-4b43-92bc-b24361068a91
*/
