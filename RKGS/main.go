package main

import (
    "encoding/base64"
    "encoding/json"
    "errors"
    "log"
    "math"
    "math/rand"
    "net"
    "strconv"
    "time"
    "github.com/gomodule/redigo/redis"
)


func main(){

    log.Println(" Connecting to redis databse")
    db_pool_var = &redis.Pool{
        MaxIdle:     20,
        IdleTimeout: 2 * time.Second,
        Dial: func() (redis.Conn, error) {
            return redis.Dial("tcp", "localhost:6379")
        },
    }

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
        n,Caddr,err := UDPConn.ReadFromUDP(buffer)
        Check_error(err)
        new_job := Job{
            Conn : UDPConn,
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
        if v.Work == "generate"{
            go generate_url(v,job)
        }else if v.Work == "check"{
            go check_if_url_exists_db(v)
        }
    }   
}


//Generates BASE64 strings as we need to use this same key for indexing and url fetching
func generate_url(job_obj Job_split, job_raw Job){
    needed_len, err:= strconv.Atoi(job_obj.Len)
    if err != nil{
        reply_job := Job_split{
            Work : "error",
            Pool : job_obj.Pool,
            Len : "",
            Url : "",
            Error : "INVALIDLEN",
        }
        json_reply_string_byte,_ := json.Marshal(reply_job);
        job_raw.Conn.WriteToUDP(json_reply_string_byte,job_raw.CAddr)
    }
    byte_len := math.Ceil((float64(needed_len)*6.0)/8.0); //6 bits for each char

    for{
        random_byte_buff := make([]byte, int(byte_len))
        rand.Seed(time.Now().UnixNano())
        rand.Read(random_byte_buff);
        url := base64.URLEncoding.EncodeToString(random_byte_buff)[0:needed_len] //to keep strict to len needs
        //EncodeToString is Base64 encoding without "/" being part of encoding scheme

        //Salting to check if this is unique in databse, optimised way to handle pools
        salted_url := url+"_"+job_obj.Pool;

        is_unique, err := check_url_is_unique_db(salted_url)
        if err!=nil{
            reply_job := Job_split{
                Work : "error",
                Pool : job_obj.Pool,
                Len : "",
                Url : "",
                Error : "INTERNALEROR",
            }
            json_reply_string_byte,_ := json.Marshal(reply_job);
            job_raw.Conn.WriteToUDP(json_reply_string_byte,job_raw.CAddr)
            return;
        }else{
            if is_unique{
                err := insert_url_db(salted_url)
                if err!=nil{
                    reply_job := Job_split{
                        Work : "error",
                        Pool : job_obj.Pool,
                        Len : "",
                        Url : "",
                        Error : "INTERNALEROR",
                    }
                    json_reply_string_byte,_ := json.Marshal(reply_job);
                    job_raw.Conn.WriteToUDP(json_reply_string_byte,job_raw.CAddr)
                    return;
                }
                reply_job := Job_split{
                    Work : "generate",
                    Pool : job_obj.Pool,
                    Len : job_obj.Len,
                    Url : url,
                    Error : "",
                }
                json_reply_string_byte,_ := json.Marshal(reply_job);
                job_raw.Conn.WriteToUDP(json_reply_string_byte,job_raw.CAddr)
                return;
            }else{
                continue
            }
        }

    }
}


func check_url_is_unique_db(salted_url string) (bool,error){
    pool_member := db_pool_var.Get()
    defer pool_member.Close()

    used, err := redis.String(pool_member.Do("HGET",salted_url,"used"))
    if err==nil{
        if used != "true"{
            return false,nil
        }else{
            return false,nil
        }
    }else{
        Check_error(err)
        return false,errors.New("databse error")
    }

}


func insert_url_db(salted_url string) error{
    pool_member := db_pool_var.Get();
    defer pool_member.Close()


    _,err := pool_member.Do("HMSET",salted_url,"used","true")

    if err!=nil{
        Check_error(err)
        return err
    }else{
        return nil
    }
}

func check_if_url_exists_db(check_job Job_split) (bool,error){
    salted_url := check_job.Url+"_"+check_job.Pool;

    pool_member := db_pool_var.Get()
    defer pool_member.Close()


    used, err := redis.String(pool_member.Do("HGET",salted_url,"used"))
    if err==nil{
        if used != "true"{
            return false,nil
        }else{
            return true,nil
        }
    }else{
        Check_error(err)
        return false,errors.New("databse error")
    }


}
