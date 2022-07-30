package main

import (
	"encoding/json"
	"log"
	"net"
)


func main(){
    log.Println(" Starting server ")
    UDPConn, err := net.ListenUDP("udp", &Addr)
    if !Check_error(err){
        log.Println(" Server started and listening on port 5001")
    }

    //initalizing variables
    Common_job_buffer = make(chan Job,1000000);
    splitter_buffer[0] = make(chan Job, 10000)
    splitter_buffer[1] = make(chan Job, 10000)
    splitter_buffer[2] = make(chan Job, 10000)
    splitter_buffer[3] = make(chan Job, 10000)


    //Spawning needed thread pools
    go conn_handlers(UDPConn);
    go conn_handlers(UDPConn);
    go conn_handlers(UDPConn);
    go conn_handlers(UDPConn);
    go job_splitter_load_balancer(Common_job_buffer);
    go job_splitter(splitter_buffer[0])
    go job_splitter(splitter_buffer[1])
    go job_splitter(splitter_buffer[2])
    go job_splitter(splitter_buffer[3])
    Wg.Add(1)
    Wg.Wait()

}

func conn_handlers(UDPConn *net.UDPConn){
    for{
        buffer := make([]byte,1000)
        n,Caddr,err := UDPConn.ReadFrom(buffer)
        Check_error(err)
        new_job := Job{
            buffer : buffer,
            CAddr : Caddr,
            len : n,
        }
        Common_job_buffer <- new_job

    }
}

func job_splitter_load_balancer(Common_job_buffer chan Job){
    var thread = 0;
    for{
        new_job := <- Common_job_buffer
        splitter_buffer[thread] <- new_job
        if thread == 3{
            thread = 0
        }
        thread++
    }
}

func job_splitter(thread_splitter_buffer chan Job){
    for{
        var v Job_split
        job  := <- thread_splitter_buffer
        log.Println(string(job.buffer))
        json.Unmarshal([]byte(string(job.buffer[:job.len])), &v)
    }   
}
