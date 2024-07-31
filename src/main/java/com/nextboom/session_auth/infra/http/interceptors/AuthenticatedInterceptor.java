package com.nextboom.session_auth.infra.http.interceptors;

import java.time.LocalDateTime;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.stereotype.Component;
import org.springframework.web.method.HandlerMethod;
import org.springframework.web.servlet.HandlerInterceptor;

import com.nextboom.session_auth.infra.http.Authenticated;

import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;


@Component
public class AuthenticatedInterceptor implements HandlerInterceptor {

  @Autowired
  private RedisTemplate<String, String> redisTemplate;

  @Override
  public boolean preHandle(HttpServletRequest request, HttpServletResponse response, Object handler) throws Exception {
    if (handler instanceof HandlerMethod) {
      HandlerMethod handlerMethod = (HandlerMethod) handler;
      Authenticated authenticated = handlerMethod.getMethodAnnotation(Authenticated.class);

      if (authenticated != null) {
        String authHeader = request.getHeader("Authorization");
        if (authHeader == null || !authHeader.startsWith("Bearer ")) {
          response.setStatus(HttpServletResponse.SC_UNAUTHORIZED);
          return false;
        }

        String token = authHeader.substring(7);
        String userId = redisTemplate.opsForValue().get("session:" + token);

        if (userId == null) {
          response.setStatus(HttpServletResponse.SC_UNAUTHORIZED);
          return false;
        }
        redisTemplate.opsForHash().put("session_data:" + token, "lastAccessAt", LocalDateTime.now().toString());
      }
    }
    
    return true;
  }
}
