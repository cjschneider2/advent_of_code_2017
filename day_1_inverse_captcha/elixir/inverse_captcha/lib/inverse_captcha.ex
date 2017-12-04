defmodule InverseCaptcha do
  @moduledoc """
  Documentation for InverseCaptcha.
  """

  @doc """
  Hello world.

  ## Examples

      iex> InverseCaptcha.hello
      :world

  """
  def hello do
    :world
  end

  def inverse_captcha_part_1(input_string) do
    out = input_string
          |> String.split
          |> Enum.map(&String.to_integer/1)
  end

  def do_part_1(int_list) do

  end

end
