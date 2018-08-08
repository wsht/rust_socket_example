<?php

$socket = socket_create(AF_INET,SOCK_STREAM,SOL_TCP);

socket_connect($socket, '127.0.0.1', 12306);
while (true){
	$msg = "test\n";
//	socket_send($socket, $msg, strlen($msg), 0);
	socket_write($socket, $msg, strlen($msg));
	var_dump(socket_last_error($socket));
	sleep(1);
}





