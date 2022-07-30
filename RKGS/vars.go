package main;



import(
    "log"
    "net"
    "sync"
)


func Check_error(err error) bool{
    if err!=nil{
        log.Println(" Error :", err)
        return true
    }
    return false
}


var Addr =  net.UDPAddr{
    Port : 5001,
    IP : net.ParseIP("0.0.0.0"),
}

type Job struct{
    Conn *net.UDPConn
    buffer []byte
    CAddr *net.UDPAddr
    len int

}

type Job_split struct{
    Work string `json:"Work"`
    Pool string `json:"Pool"`
    Len string  `json:"Len"`
    Url string  `json:"Url"`
    Error string  `json:"Error"`
}

var Common_job_buffer chan Job
var splitter_buffer [4]chan Job

var Wg sync.WaitGroup


