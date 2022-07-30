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
    buffer []byte
    CAddr net.Addr
    len int

}

type Job_split struct{
    Work string `json:"Work"`
    Pool string `json:"Pool"`
    Url string `json:"Url"`
}

var Common_job_buffer chan Job
var splitter_buffer [4]chan Job

var Wg sync.WaitGroup


