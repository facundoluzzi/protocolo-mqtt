# Template

## TODO

1. cerrar el listener luego de cada setup
2. decidir si enviar un mensaje de error previo a cortar la conexión

## Keep alive

1. Es un intervalo de tiempo medido en segundos.
2. Está representado en dos bytes (MSB y LSB).
3. Es el máximo intervalo de tiempo permitido para que el cliente empieza a enviar el siguiente paquete.
4. El cliente es el responsable de controlar los envios. Si el cliente no enviará ningún paquete, debe enviar un PINGREQ packet en su lugar.
5. Si el keep alive no es cero y el server no recibe ningún paquete en 1 vez y medio el período de tiempo determinado por el keep alive, debe cerrar la conexión.
6. Si el cliente no recibe un PINGRESP packet en un período de tiempo razonable después de haber enviado el pingreq, debería (no necesariamente) cerrar la conexión con el server.
7. Si el keep alive es 0, el server no requiere desconectar el cliente. Pero no indica que debe mantenerlo abierto tampoco (a chequear)
