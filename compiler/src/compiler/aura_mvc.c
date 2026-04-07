#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif

#ifdef _WIN32
    #include <WinSock2.h>
    #include <Windows.h>
    #include <WS2tcpip.h>
    #include <stdio.h>
    #include <stdlib.h>
    #include <string.h>
#else
    #include <sys/socket.h>
    #include <netinet/in.h>
    #include <unistd.h>
    #include <stdio.h>
    #include <stdlib.h>
    #include <string.h>
#endif

typedef char* (*AuraControllerFunc)(void* instance, long long param);

typedef struct {
    char path[64];
    AuraControllerFunc func;
} Route;

static Route registry[32];
static int route_count = 0;

void aura_mvc_register(const char* path, AuraControllerFunc func_ptr) {
    if (route_count < 32) {
        // strncpy yerine güvenli kopyalama
        size_t i = 0;
        for (i = 0; i < 63 && path[i] != '\0'; i++) {
            registry[route_count].path[i] = path[i];
        }
        registry[route_count].path[i] = '\0';
        registry[route_count].func = func_ptr;
        route_count++;
    }
}

void aura_mvc_serve(long long sock_ll, void* instance) {
    if (sock_ll == -1) return;

    printf("MVC Server listening...\n");
    while (1) {
#ifdef _WIN32
        long long client_sock = (long long)accept((SOCKET)sock_ll, NULL, NULL);
#else
        long long client_sock = (long long)accept((int)sock_ll, NULL, NULL);
#endif
        if (client_sock == -1) continue;

        char buffer[1024] = {0};
#ifdef _WIN32
        recv((SOCKET)client_sock, buffer, 1024, 0);
#else
        recv((int)client_sock, buffer, 1024, 0);
#endif

        // Basit routing
        char* path_start = strstr(buffer, "GET /");
        if (path_start) {
            path_start += 5;
            char* path_end = strpbrk(path_start, " \r\n");
            if (path_end) {
                char method_name[64] = {0};
                size_t len = path_end - path_start;
                char* q_mark = strchr(path_start, '?');
                if (q_mark && q_mark < path_end) {
                    len = q_mark - path_start;
                }
                if (len > 63) len = 63;
                
                // Safe copy
                for (size_t i = 0; i < len; i++) {
                    method_name[i] = path_start[i];
                }
                method_name[len] = '\0';

                // Registry'de ara
                int found = 0;
                for (int i = 0; i < route_count; i++) {
                    if (strcmp(registry[i].path, method_name) == 0) {
                        // Parametre parse (basitçe ?'dan sonraki = sonrasını alıyoruz)
                        long long param = 0;
                        char* q = strchr(path_start, '?');
                        if (q && q < path_end) {
                            char* eq = strchr(q, '=');
                            if (eq && eq < path_end) {
                                sscanf(eq + 1, "%lld", &param);
                            }
                        }

                        char* response_body = registry[i].func(instance, param);
                        
                        char header[256];
                        int h_len = sprintf(header, "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: %zu\r\n\r\n", strlen(response_body));
                        
#ifdef _WIN32
                        send((SOCKET)client_sock, header, h_len, 0);
                        send((SOCKET)client_sock, response_body, (int)strlen(response_body), 0);
#else
                        send((int)client_sock, header, h_len, 0);
                        send((int)client_sock, response_body, (int)strlen(response_body), 0);
#endif
                        found = 1;
                        break;
                    }
                }

                if (!found) {
                    char error404[] = "HTTP/1.1 404 Not Found\r\n\r\n{\"error\":\"Route not found\"}";
#ifdef _WIN32
                    send((SOCKET)client_sock, error404, (int)strlen(error404), 0);
#else
                    send((int)client_sock, error404, (int)strlen(error404), 0);
#endif
                }
            }
        }

#ifdef _WIN32
        closesocket((SOCKET)client_sock);
#else
        close((int)client_sock);
#endif
    }
}
