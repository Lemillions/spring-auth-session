package com.nextboom.session_auth.infra.http.configs;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.context.annotation.Configuration;
import org.springframework.web.method.support.HandlerMethodArgumentResolver;
import org.springframework.web.servlet.config.annotation.InterceptorRegistry;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer;

import com.nextboom.session_auth.infra.http.interceptors.AuthenticatedInterceptor;
import com.nextboom.session_auth.infra.http.resolvers.AuthenticatedArgumentResolver;

import java.util.List;

@Configuration
public class WebConfig implements WebMvcConfigurer {

  @Autowired
  private AuthenticatedInterceptor authenticatedInterceptor;

  @Autowired
  private AuthenticatedArgumentResolver authenticatedArgumentResolver;

  @Override
  public void addInterceptors(InterceptorRegistry registry) {
    registry.addInterceptor(authenticatedInterceptor);
  }

  @Override
  public void addArgumentResolvers(List<HandlerMethodArgumentResolver> resolvers) {
    resolvers.add(authenticatedArgumentResolver);
  }
}
