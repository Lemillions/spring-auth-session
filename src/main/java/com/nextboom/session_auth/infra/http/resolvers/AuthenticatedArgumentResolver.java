package com.nextboom.session_auth.infra.http.resolvers;

import java.util.Map;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.core.MethodParameter;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.stereotype.Component;
import org.springframework.web.bind.support.WebDataBinderFactory;
import org.springframework.web.context.request.NativeWebRequest;
import org.springframework.web.method.support.HandlerMethodArgumentResolver;
import org.springframework.web.method.support.ModelAndViewContainer;

import com.nextboom.session_auth.infra.exceptions.UnauthorizedException;
import com.nextboom.session_auth.infra.http.Authenticated;
import com.nextboom.session_auth.infra.redis.Session;

@Component
public class AuthenticatedArgumentResolver implements HandlerMethodArgumentResolver {

  @Autowired
  private RedisTemplate<String, String> redisTemplate;

  @Override
  public boolean supportsParameter(MethodParameter parameter) {
    return parameter.hasMethodAnnotation(Authenticated.class);
  }

  @Override
  public Session resolveArgument(MethodParameter parameter, ModelAndViewContainer mavContainer,
      NativeWebRequest webRequest, WebDataBinderFactory binderFactory) throws Exception {
    String authHeader = webRequest.getHeader("Authorization");

    if (authHeader == null || !authHeader.startsWith("Bearer ")) {
      throw new UnauthorizedException("Não autorizado");
    }

    String token = authHeader.substring(7);

    Map<Object, Object> sessionData = redisTemplate.opsForHash().entries(Session.DATA_PREFIX + token);

    if (sessionData == null || sessionData.isEmpty()) {
      throw new UnauthorizedException("Não autorizado");
    }

    return new Session(
        token,
        sessionData.get("userId").toString(),
        sessionData.get("createdAt").toString(),
        sessionData.get("expiresAt").toString(),
        sessionData.get("lastAccessAt").toString(),
        sessionData.get("ip").toString(),
        sessionData.get("userAgent").toString());
  }
}
