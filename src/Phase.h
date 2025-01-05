// Copyright (C) 2020-2024 Parabola Research Limited
// SPDX-License-Identifier: MPL-2.0

#pragma once

#include "Assert.h"

#include <algorithm>
#include <array>

#include <complex>
#include <cstdint>
#include <limits>

// Mathematical constants
namespace constants {
    constexpr double pi = 3.14159265358979323846;
}

namespace Bungee::Phase {

typedef int16_t Type;

template <typename T2, typename T1>
T2 cast(T1 t1)
{
	if constexpr (sizeof(T2) > sizeof(T1))
		return T2(t1) << (8 * sizeof(T2) - 8 * sizeof(T1));
	else
		return T2(t1 >> (8 * sizeof(T1) - 8 * sizeof(T2)));
}

template <typename T = Type>
static inline constexpr float toRevolutions(T phase)
{
	constexpr auto shift = 8 * sizeof(T);
	constexpr auto k = float(1. / (1ull << shift));
	return phase * k;
}

template <typename T = Type>
static inline constexpr T fromRevolutions(float revolutions)
{
	constexpr auto shift = 8 * sizeof(T);
	const float k = float(1ull << shift);
	const T phase = T(k * revolutions);
	BUNGEE_ASSERT1(phase == T(int64_t(k * revolutions)));
	return phase;
}

template <typename T = Type>
static inline constexpr float toRadians(T phase)
{
	constexpr auto shift = 8 * sizeof(T);
	constexpr auto k = float((2 * constants::pi) / (1ull << shift));
	return phase * k;
}

template <typename T = Type>
static inline constexpr T fromRadians(float radians)
{
	constexpr auto shift = 8 * sizeof(T);
	constexpr float k = float((1ull << shift) / (2 * constants::pi));
	const T phase = T(k * radians);
	return phase;
}

template <typename T = Type>
static inline constexpr T fromTime(double time, int log2Period)
{
	const auto shift = 8 * sizeof(T) - log2Period;
	const double phase = double(1ull << shift) * time;
	return T(int64_t(phase));
}

} // namespace Bungee::Phase
