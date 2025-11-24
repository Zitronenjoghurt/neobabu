<script lang="ts" setup></script>

<template>
  <div class="landing-page flex align-items-center justify-content-center select-none">
    <div class="floating-icons">
      <i class="pi pi-discord floating-icon" style="top: 10%; left: 10%"></i>
      <i class="pi pi-star floating-icon" style="top: 20%; right: 15%"></i>
      <i class="pi pi-bolt floating-icon" style="bottom: 15%; left: 20%"></i>
      <i class="pi pi-sparkles floating-icon" style="bottom: 20%; right: 10%"></i>
    </div>

    <div class="content-wrapper text-center">
      <div class="mb-6">
        <div class="mb-4">
          <i class="pi pi-wrench text-6xl text-primary rotating-icon"></i>
        </div>
        <h1 class="text-5xl md:text-6xl font-bold mb-3 fade-in-up">Under Construction</h1>
        <p class="text-xl md:text-2xl text-color-secondary mb-4 fade-in-up delay-1">
          I am currently building a silly little discord bot and this is its web dashboard
        </p>
        <p class="text-lg text-color-secondary fade-in-up delay-2">
          Login with Discord in the top right to get early access and explore what I am working on
        </p>
      </div>

      <div class="grid mt-6">
        <div class="col-12 md:col-4 fade-in-up delay-3">
          <Card class="feature-card h-full">
            <template #content>
              <i class="pi pi-chart-line text-3xl text-primary mb-2"></i>
              <h3 class="font-semibold mb-2">Level System</h3>
              <p class="text-sm text-color-secondary">Global, server-wide and customizable</p>
            </template>
          </Card>
        </div>
        <div class="col-12 md:col-4 fade-in-up delay-4">
          <Card class="feature-card h-full">
            <template #content>
              <i class="pi pi-face-smile text-3xl text-primary mb-2"></i>
              <h3 class="font-semibold mb-2">Fun</h3>
              <p class="text-sm text-color-secondary">Minigames and other silly things</p>
            </template>
          </Card>
        </div>
        <div class="col-12 md:col-4 fade-in-up delay-5">
          <Card class="feature-card h-full">
            <template #content>
              <i class="pi pi-cog text-3xl text-primary mb-2"></i>
              <h3 class="font-semibold mb-2">Utilities</h3>
              <p class="text-sm text-color-secondary">Youtube, NASA APOD Notifications, and more</p>
            </template>
          </Card>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.landing-page {
  position: relative;
  overflow-x: hidden;
  overflow-y: auto;
}

.floating-icons {
  position: absolute;
  width: 100%;
  height: 100%;
}

.floating-icon {
  position: absolute;
  font-size: 4rem;
  color: var(--p-primary-color);
  animation: float 6s ease-in-out infinite;
  pointer-events: none;
  opacity: 0.1;
}

@keyframes float {
  0%,
  100% {
    transform: translateY(0px) rotate(0deg);
  }
  50% {
    transform: translateY(-20px) rotate(5deg);
  }
}

.rotating-icon {
  animation: rotate-wobble 3s ease-in-out infinite;
  display: inline-block;
}

@keyframes rotate-wobble {
  0%,
  100% {
    transform: rotate(0deg);
  }
  25% {
    transform: rotate(-15deg);
  }
  75% {
    transform: rotate(15deg);
  }
}

.fade-in-up {
  animation: fadeInUp 0.8s ease-out forwards;
  opacity: 0;
}

.delay-1 {
  animation-delay: 0.2s;
}
.delay-2 {
  animation-delay: 0.4s;
}
.delay-3 {
  animation-delay: 0.6s;
}
.delay-4 {
  animation-delay: 0.8s;
}
.delay-5 {
  animation-delay: 1s;
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(30px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.feature-card {
  transition:
    transform 0.3s ease,
    box-shadow 0.3s ease;
}

.feature-card:hover {
  transform: translateY(-8px);
  box-shadow: 0 12px 24px rgba(0, 0, 0, 0.15);
}

.feature-card i {
  transition: transform 0.3s ease;
}

.feature-card:hover i {
  transform: scale(1.2);
}
</style>
