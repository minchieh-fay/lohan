export namespace main {
	
	export class ModelInfo {
	    name: string;
	    size: string;
	    description: string;
	    url: string;
	    filename: string;
	
	    static createFrom(source: any = {}) {
	        return new ModelInfo(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.name = source["name"];
	        this.size = source["size"];
	        this.description = source["description"];
	        this.url = source["url"];
	        this.filename = source["filename"];
	    }
	}
	export class ServerInfo {
	    id: string;
	    name: string;
	    ip: string;
	    ssh_port: number;
	    username: string;
	    password?: string;
	    private_key?: string;
	    use_root: boolean;
	    sudo_method: string;
	    sudo_pass?: string;
	
	    static createFrom(source: any = {}) {
	        return new ServerInfo(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.id = source["id"];
	        this.name = source["name"];
	        this.ip = source["ip"];
	        this.ssh_port = source["ssh_port"];
	        this.username = source["username"];
	        this.password = source["password"];
	        this.private_key = source["private_key"];
	        this.use_root = source["use_root"];
	        this.sudo_method = source["sudo_method"];
	        this.sudo_pass = source["sudo_pass"];
	    }
	}

}

export namespace ssh {
	
	export class Client {
	    Conn: any;
	
	    static createFrom(source: any = {}) {
	        return new Client(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.Conn = source["Conn"];
	    }
	}

}

